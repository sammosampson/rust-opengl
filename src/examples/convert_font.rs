use crate::render_gl::font;

pub fn run() -> Result<(), failure::Error> {
    font::convert_font_glyph_images_to_column_images("/Users/Papa/Downloads/segoeui/", (96, 96), 500).unwrap();
    Ok(())
}

