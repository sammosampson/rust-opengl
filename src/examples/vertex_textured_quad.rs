use crate::render_gl;
use crate::resources;
use crate::window;

pub fn run() -> Result<(), failure::Error> {
    let window_manager = window::WindowManager
        ::init()?;

    let mut window_builder = window::WindowBuilder
        ::new(&window_manager)?;

    let window = window_builder
        .with_title("Vertex textured quad")
        .with_width(900)
        .with_height(700)
        .build()?;

    let gl = gl::Gl
        ::load_with(|process_name|window_manager.get_process_address(process_name));

    let res = resources::Resources
        ::from_relative_exe_path(std::path::Path::new("assets-07"))?;
    
    let quad = quad(&res, &gl)?;
    let viewport = render_gl::viewport::Viewport::for_window(1800, 1400);
    let colour_buffer = render_gl::buffer::ColorBuffer::from_colour((0.3, 0.3, 0.5));

    viewport.set_used(&gl);
    colour_buffer.set_used(&gl);

    window_manager.loop_until_window_quit(|_,__| {
        colour_buffer.clear(&gl); 
        quad.render(&gl);
        window.swap();
    })?;
    Ok(())
}

pub fn quad(res: &resources::Resources, gl: &gl::Gl)
    -> Result<render_gl::rendering::TexturedVertexPrimitiveRenderer, failure::Error> {
    let shader_program = render_gl::Program
        ::from_res(&gl, &res, &["shaders/second/textured_primitive.vert", "shaders/second/textured_primitive.frag"])?;

    let vertices: Vec<render_gl::vertices::TexturedVertex> = vec![
        render_gl::vertices::TexturedVertex::with_position_and_texture_coordinate((1400.0, 1400.0, 0.0).into(), (1.0, 1.0).into()),
        render_gl::vertices::TexturedVertex::with_position_and_texture_coordinate((1400.0, 0.0, 0.0).into(), (1.0, 0.0).into()),
        render_gl::vertices::TexturedVertex::with_position_and_texture_coordinate((0.0, 1400.0, 0.0).into(), (0.0, 1.0).into()),
        render_gl::vertices::TexturedVertex::with_position_and_texture_coordinate((0.0, 0.0, 0.0).into(), (0.0, 0.0).into()),
    ];

    render_gl::rendering::TexturedVertexPrimitiveRenderer::new(gl, shader_program, vertices, 4)
}