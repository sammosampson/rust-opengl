
use crate::resources::{Resources, Error};
use crate::render_gl::shader::Program;
use crate::render_gl::texture::{Texture, TextureType};
use image::imageops::crop;
use image::RgbaImage;
use std::path::{PathBuf};

pub fn convert_font_glyph_images_to_column_images(path: &str, glyph_dimension: (u32, u32), glyph_count: usize) -> Result<(), Error> {
    let mut column_image = image::RgbaImage::new(glyph_dimension.0, glyph_count as u32 * glyph_dimension.1);
    for glyph_index in 0..glyph_count {
        let glyph_image_path: PathBuf = PathBuf::from(path);
        let glyph_image_path = glyph_image_path.join(format!("g{}.png", glyph_index + 1));
        let glyph_image = image::open(glyph_image_path)?.into_rgba8();
        image::imageops::overlay(&mut column_image, &glyph_image, 0, glyph_index as u32 * glyph_dimension.1);
    }
    column_image.save("segoeui-1.png").unwrap();
    Ok(())
}

pub struct FontSampler {
    buffer_number: u32,
    image: Vec<u8>,
    texture: Texture::<(u8, u8, u8, u8)>,
    glyph_dimensions: (i32, i32),
    glyph_count: i32
}

impl FontSampler {
    pub fn from_font_resource(gl: &gl::Gl, res: &Resources, font_image_name: &str, glyph_count: i32, glyph_dimensions: (i32, i32), buffer_number: u32) -> Result<FontSampler, Error> {
        println!("loading {}...", font_image_name);
        let raw_image = res.load_image(font_image_name)?.to_rgba8();
        println!("loaded {}...", font_image_name);
        Ok(
            FontSampler {
                buffer_number,
                image: raw_image.into_raw(),
                texture: Texture::<(u8, u8, u8, u8)>::new(gl, TextureType::Texture2dArray, buffer_number),
                glyph_dimensions,
                glyph_count
        })
    }

    pub fn send_to_shader_property(&mut self, program: &Program, property_name: &str) {
        self.texture.bind();
        self.texture.set_texture_storage(1, self.glyph_dimensions, self.glyph_count);
        self.texture.set_sub_texture(&self.image, 0, (0, 0), self.glyph_dimensions, 0, self.glyph_count);
        self.texture.set_linear_texture_filter();

        program.set_used();
        program.set_uniform_1i(property_name, self.buffer_number as i32);
    } 
}