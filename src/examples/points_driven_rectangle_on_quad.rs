use crate::render_gl::{
    rendering:: {
        Blender,
        QuadShapeVertexPrimitiveRenderer
    }, 
    vertices::QuadShapeVertex,
    viewport::Viewport,
    buffer::{
        ColorBuffer
    },
    texture::{
        RGBA32FUniformBuffer,
        R32FUniformBuffer
    },
    Program
};
use crate::resources;
use crate::window;

pub fn run() -> Result<(), failure::Error> {
    let window_manager = window::WindowManager
        ::init()?;

    let mut window_builder = window::WindowBuilder
        ::new(&window_manager)?;

    let window = window_builder
        .with_title("Rect on quad")
        .with_width(900)
        .with_height(700)
        .build()?;

    let gl = gl::Gl
        ::load_with(|process_name|window_manager.get_process_address(process_name));

    let res = resources::Resources
        ::from_relative_exe_path(std::path::Path::new("assets-07"))?;
    
    let renderer = quad(&res, &gl)?;
    renderer.program().set_used();
    renderer.program().set_uniform_vec2f("uResolution", 1800.0, 1400.0);

    let mut inner_colour_uniform = RGBA32FUniformBuffer::new(&gl, 0);
    inner_colour_uniform.add_data((0.5, 0.5, 0.5, 1.0));
    inner_colour_uniform.send_to_shader_property(&renderer.program(), "inner_colour_buffer");

    let mut outer_colour_uniform = RGBA32FUniformBuffer::new(&gl, 1);
    outer_colour_uniform.add_data((1.0, 1.0, 1.0, 1.0));
    outer_colour_uniform.send_to_shader_property(&renderer.program(), "outer_colour_buffer");
   
    let mut stroke_width_uniform = R32FUniformBuffer::new(&gl, 2);
    stroke_width_uniform.add_data(0.05);
    stroke_width_uniform.send_to_shader_property(&renderer.program(), "stroke_width_buffer");

    let mut corner_radii_uniform = RGBA32FUniformBuffer::new(&gl, 3);
    corner_radii_uniform.add_data((0.1, 0.3, 0.4, 0.2));
    corner_radii_uniform.send_to_shader_property(&renderer.program(), "corner_radii_buffer");
   
    let viewport = Viewport::for_window(1800, 1400);
    let colour_buffer = ColorBuffer::from_colour((0.3, 0.3, 0.5));
    let blender = Blender::new();

    viewport.set_used(&gl);
    colour_buffer.set_used(&gl);
    colour_buffer.clear(&gl);
    blender.set_used(&gl);
    renderer.render(&gl);
    blender.set_unused(&gl);
    window.swap();

    window_manager.loop_until_window_quit(|_,__| {
        
    })?;
    Ok(())
}

pub fn quad(res: &resources::Resources, gl: &gl::Gl)
    -> Result<QuadShapeVertexPrimitiveRenderer, failure::Error> {
    let shader_program = Program
        ::from_res(&gl, &res, &[
            "shaders/third/point_and_size.vert",
            "shaders/third/quad.geom",
            "shaders/third/rectangle.frag"])?;

    let points: Vec<QuadShapeVertex> = vec![
        QuadShapeVertex::with_position_and_size((100.0, 100.0).into(), (100.0, 100.0).into(), (0.0, 0.0, 0.0).into()),
        QuadShapeVertex::with_position_and_size((200.0, 100.0).into(), (200.0, 200.0).into(), (0.0, 0.0, 0.0).into()),
        QuadShapeVertex::with_position_and_size((100.0, 400.0).into(), (50.0, 50.0).into(), (0.0, 0.0, 0.0).into()),
        QuadShapeVertex::with_position_and_size((800.0, 100.0).into(), (25.0, 25.0).into(), (0.0, 0.0, 0.0).into()),
        QuadShapeVertex::with_position_and_size((900.0, 700.0).into(), (300.0, 300.0).into(), (0.0, 0.0, 0.0).into())
    ];

    QuadShapeVertexPrimitiveRenderer::new(gl, shader_program, points, 5)
}