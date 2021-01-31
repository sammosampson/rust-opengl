use failure::err_msg;
use std::time::Instant;

pub struct WindowManager {
    sdl: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem
}

impl WindowManager {
    pub fn init() -> Result<Self, failure::Error>  {
        let sdl = sdl2::init().map_err(err_msg)?;
        
        let video_subsystem = sdl.video().map_err(err_msg)?;

        let gl_window_attributes = video_subsystem.gl_attr();
        gl_window_attributes.set_context_profile(sdl2::video::GLProfile::Core);
        gl_window_attributes.set_context_version(4, 1);

        let window_manager = Self {
            sdl,
            video_subsystem,
        };

        Ok(window_manager)
    }

    pub fn get_process_address(&self, process_name: &str) -> *const std::os::raw::c_void {
        self.video_subsystem.gl_get_proc_address(process_name) as *const std::os::raw::c_void
    }

    pub fn loop_until_window_quit<F>(&self, to_loop: F) -> Result<(), failure::Error> where F: Fn(f32, f32) {
        let total_time = Instant::now();
        let mut time = Instant::now();
        let mut event_pump = self.sdl.event_pump().map_err(err_msg)?;
        'main: loop {
            println!("Doing stuff");
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }
            let delta = time.elapsed().as_secs_f32();
            let total_time_delta = total_time.elapsed().as_secs_f32();
            time = Instant::now();
            to_loop(total_time_delta, delta);
        }
        Ok(())
    }
}

pub struct Window {
    wrapped: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext
}

impl Window {
    pub fn new(wrapped: sdl2::video::Window) -> Result<Self, failure::Error> {
        let _gl_context = wrapped.gl_create_context().map_err(err_msg)?;
        let window = Self {
            wrapped,
            _gl_context
        };
        Ok(window)
    }

    pub fn swap(&self) {
        self.wrapped.gl_swap_window();
    }
}

pub struct WindowBuilder<'a> {
    window_manager: &'a WindowManager,
    title: &'a str,
    width: u32,
    height: u32
}

impl<'a> WindowBuilder<'a> {
    pub fn new(window_manager: &'a WindowManager) -> Result<Self, failure::Error> {
        let window_builder = Self {
            window_manager,
            title: Default::default(),
            width: Default::default(),
            height: Default::default(),
        };
        Ok(window_builder)
    }

    pub fn with_title(&mut self, title: &'a str) -> &'a mut WindowBuilder {
        self.title = title;
        self
    }

    pub fn with_width(&mut self, width: u32) -> &'a mut WindowBuilder {
        self.width = width;
        self
    }

    pub fn with_height(&mut self, height: u32) -> &'a mut WindowBuilder {
        self.height = height;
        self
    }

    pub fn build(&self) -> Result<Window, failure::Error> {
        let window = Window::new(
            self.window_manager.video_subsystem
                .window(self.title, self.width, self.height)
                .opengl()
                .resizable()
                .build()
                .map_err(err_msg)?
        )?;
        Ok(window)
    }
}