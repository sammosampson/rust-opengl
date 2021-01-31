pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Viewport {
    pub fn for_window(width: i32, height: i32) -> Viewport {
        Viewport {
            x: 0,
            y: 0,
            width,
            height
        }
    }

    pub fn update_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_used(&self, gl: &gl::Gl) {
        unsafe {
            gl.Viewport(self.x, self.y, self.width, self.height);
        }
    }
}