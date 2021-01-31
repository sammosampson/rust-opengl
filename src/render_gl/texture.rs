use crate::render_gl::buffer::TextureBuffer;
use crate::render_gl::shader::Program;
use crate::resources::{Resources, Error};

pub struct Texture<T> where T : TextureLayoutType {
    gl: gl::Gl,
    texture_id: gl::types::GLuint,
    texture_number: u32,
    texture_type: gl::types::GLuint,
    layout_type: gl::types::GLuint,
    _marker: ::std::marker::PhantomData<T>
}

impl<T> Texture<T> where T : TextureLayoutType {
    pub fn new(gl: &gl::Gl, texture_type: TextureType, texture_number: u32) -> Texture<T> {
        let mut texture_id: gl::types::GLuint = 0;
        unsafe {
            gl.GenTextures(1, &mut texture_id);
        }

        Texture::<T> {
            gl: gl.clone(),
            texture_id,
            texture_number,
            texture_type: texture_type.get_gl_type(),
            layout_type: T::get_gl_layout_type(),
            _marker: ::std::marker::PhantomData
        }
    }

    pub fn activate(&self) {
        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0 + self.texture_number);
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindTexture(self.texture_type, self.texture_id);
        }
    }

    pub fn use_buffer(&self, buffer: &TextureBuffer) {
        unsafe {
            self.gl.TexBuffer(self.texture_type, self.layout_type, buffer.gl_id());
        }
    }
    
    pub fn set_texture(&self, image: &[u8], dimensions: (i32, i32)) {
        unsafe {
            self.gl.TexImage2D(
                self.texture_type,
                0,
                gl::RGBA as i32,
                dimensions.0,
                dimensions.1 ,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.as_ptr() as *const std::os::raw::c_void);
        }
    }

    pub fn set_texture_storage(&self, mip_level_count: i32, dimensions: (i32, i32), layer_count: i32) {
        unsafe {
            self.gl.TexStorage3D(
                self.texture_type, 
                mip_level_count, 
                self.layout_type, 
                dimensions.0,
                dimensions.1,
                layer_count);
        }
    }

    pub fn set_sub_texture(&self, image: &[u8], mip_level: i32, offsets: (i32, i32), dimensions: (i32, i32), layer: i32, layer_count: i32) {
        unsafe {
            self.gl.TexSubImage3D(
                self.texture_type,
                mip_level,
                offsets.0,
                offsets.1, 
                layer,
                dimensions.0,
                dimensions.1, 
                layer_count,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.as_ptr() as *const std::os::raw::c_void);
        }
    }

    pub fn set_texture_3d(&self, image: &[u8], mip_level: i32,  dimensions: (i32, i32), layer_count: i32) {
        unsafe {
            self.gl.TexImage3D(
                self.texture_type,
                mip_level,
                gl::RGBA as i32,
                dimensions.0,
                dimensions.1, 
                layer_count,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.as_ptr() as *const std::os::raw::c_void);
        }
    }

    pub fn set_linear_texture_filter(&self) {
        unsafe {
            self.gl.TexParameteri(self.texture_type, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            self.gl.TexParameteri(self.texture_type, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
    }
}

pub enum TextureType {
    Texture1d,
    Texture2d,
    Texture1dArray,
    Texture2dArray,
    TextureBuffer
}
 
impl TextureType {
    fn get_gl_type(&self) -> gl::types::GLuint {
        match self {
            TextureType::Texture1d => gl::TEXTURE_1D,
            TextureType::Texture2d => gl::TEXTURE_2D,
            TextureType::Texture1dArray => gl::TEXTURE_1D_ARRAY,
            TextureType::Texture2dArray => gl::TEXTURE_2D_ARRAY,
            TextureType::TextureBuffer => gl::TEXTURE_BUFFER,
        }
    }
}

pub trait TextureLayoutType {
    fn get_gl_layout_type() -> gl::types::GLuint;
}

impl TextureLayoutType for (f32, f32, f32, f32) {
    fn get_gl_layout_type() -> gl::types::GLuint {
        gl::RGBA32F
    }
}

impl TextureLayoutType for (u8, u8, u8, u8) {
    fn get_gl_layout_type() -> gl::types::GLuint {
        gl::RGBA8
    }
}

impl TextureLayoutType for f32 {
    fn get_gl_layout_type() -> gl::types::GLuint {
        gl::R32F
    }
}

pub struct TextureSampler {
    buffer_number: u32,
    data: Vec<u8>,
    dimensions: (i32, i32),
    texture: Texture<(u8, u8, u8, u8)>
}

impl TextureSampler {
    pub fn from_resource(gl: &gl::Gl, res: &Resources, resource_name: &str, buffer_number: u32) -> Result<TextureSampler, Error> {
        let image = res.load_image(resource_name)?.to_rgba8();
        let dimensions = image.dimensions();

        let sampler = TextureSampler {
            buffer_number,
            data: image.into_raw(),
            dimensions: (dimensions.0 as i32, dimensions.1 as i32),
            texture: Texture::<(u8, u8, u8, u8)>::new(gl, TextureType::Texture2d, buffer_number)
        };

        Ok(sampler)
    }
    
    pub fn send_to_shader_property(&self, program: &Program, property_name: &str) {
        self.texture.activate();
        self.texture.bind();
        self.texture.set_texture(&self.data, self.dimensions);
        self.texture.set_linear_texture_filter();

        program.set_used();
        program.set_uniform_1i(property_name, self.buffer_number as i32);
    } 
}

pub struct UniformBuffer<T> where T : TextureLayoutType {
    buffer_number: u32,
    data: Vec<T>,
    texture_buffer: TextureBuffer,
    texture: Texture<T>
}

impl<T> UniformBuffer<T> where T : TextureLayoutType {
    pub fn new(gl: &gl::Gl, buffer_number: u32) -> UniformBuffer::<T> {
        UniformBuffer::<T> {
            buffer_number,
            data: vec![],
            texture_buffer: TextureBuffer::new(gl),
            texture: Texture::<T>::new(gl, TextureType::TextureBuffer, buffer_number)
        }
    }

    pub fn add_data(&mut self, data: T) {
        self.data.push(data)
    }

    pub fn send_to_shader_property(&self, program: &Program, property_name: &str) {
        self.texture_buffer.bind();
        self.texture_buffer.set_draw_data(&self.data);
    
        self.texture.activate();
        self.texture.bind();
        self.texture.use_buffer(&self.texture_buffer);
        self.texture_buffer.unbind();

        program.set_used();
        program.set_uniform_1i(property_name, self.buffer_number as i32);
    } 
}

pub type RGBA32FUniformBuffer = UniformBuffer<(f32, f32, f32, f32)>;
pub type R32FUniformBuffer = UniformBuffer<f32>;