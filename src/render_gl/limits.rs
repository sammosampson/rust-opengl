
pub fn get_maximum_layers(gl: &gl::Gl) -> i32 {
    let mut max_layers: i32 = 0;
    unsafe {
        gl.GetIntegerv(gl::MAX_ARRAY_TEXTURE_LAYERS, &mut max_layers);
    }
    max_layers
}

pub fn get_maximum_texture_size(gl: &gl::Gl) -> i32 {
    let mut max_texture_size: i32 = 0;
    unsafe {
        gl.GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut max_texture_size);
    }
    max_texture_size
}

