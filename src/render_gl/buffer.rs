use gl;

pub struct ColorBuffer {
    pub colour: (f32, f32, f32, f32),
}

impl ColorBuffer {
    pub fn from_colour(colour: (f32, f32, f32)) -> ColorBuffer {
        ColorBuffer {
            colour: (colour.0, colour.1, colour.2, 1.0),
        }
    }

    pub fn set_used(&self, gl: &gl::Gl) {
        unsafe {
            gl.ClearColor(self.colour.0, self.colour.1, self.colour.2, self.colour.3);
        }
    }

    pub fn clear(&self, gl: &gl::Gl) {
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;
pub type TextureBuffer = Buffer<BufferTypeTexture>;

pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

pub struct BufferTypeTexture;
impl BufferType for BufferTypeTexture {
    const BUFFER_TYPE: gl::types::GLuint = gl::TEXTURE_BUFFER;
}

pub struct Buffer<B> where B: BufferType {
    gl: gl::Gl,
    id: gl::types::GLuint,
    _marker: ::std::marker::PhantomData<B>
}

impl<B> Buffer<B> where B: BufferType {
    pub fn new(gl: &gl::Gl) -> Buffer<B> {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }

        Buffer {
            gl: gl.clone(),
            id,
            _marker: ::std::marker::PhantomData
        }
    }
    
    pub fn bind(&self) {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    pub fn set_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                B::BUFFER_TYPE, // target
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::DYNAMIC_DRAW, // usage
            );
        }
    }

    pub fn update_draw_data<T>(&self, data: &[T], offset: usize) {
        unsafe {
            self.gl.BufferSubData(
                B::BUFFER_TYPE, // target
                (offset * ::std::mem::size_of::<T>()) as gl::types::GLintptr, // offset of data in bytes
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
            );
        }
    }

    pub fn gl_id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl<B> Drop for Buffer<B> where B: BufferType  {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.id);
        }
    }
}

pub struct VertexArray {
    gl: gl::Gl,
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new(gl: &gl::Gl) -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }

        VertexArray {
            gl: gl.clone(),
            vao
        }
    }
    
    pub fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteVertexArrays(1, &mut self.vao);
        }
    }
}