extern crate image;

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
        TextureSampler
    },
    font::FontSampler,
    Program,
    limits::*
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

    let renderer = quad(&res, &gl, (10, 50))?;
    renderer.program().set_used();
    renderer.program().set_uniform_vec2f("uResolution", 1800.0, 1400.0);

    let mut sampler = FontSampler::from_font_resource(&gl, &res, "fonts/segoeui-1.png", 500, (96, 96), 0)?;
    sampler.send_to_shader_property(&renderer.program(), "font_buffer");
    
    println!("max texture size: {}", get_maximum_texture_size(&gl));
    println!("max layers: {}", get_maximum_layers(&gl));

    let mut inner_colour_uniform = RGBA32FUniformBuffer::new(&gl, 1);
    inner_colour_uniform.add_data((0.5, 0.5, 0.5, 1.0));
    inner_colour_uniform.send_to_shader_property(&renderer.program(), "inner_colour_buffer");
   
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

pub fn quad(res: &resources::Resources, gl: &gl::Gl, glyph_count: (i32, i32))
    -> Result<QuadShapeVertexPrimitiveRenderer, failure::Error> {
    let shader_program = Program
        ::from_res(&gl, &res, &[
            "shaders/third/point_and_size.vert",
            "shaders/third/quad.geom",
            "shaders/third/text.frag"])?;

    let mut points: Vec<QuadShapeVertex> = vec!();
    for y in 0..glyph_count.1 {
        for x in 0..glyph_count.0 {
            points.push(QuadShapeVertex::with_position_and_size(
                (x as f32 * 64.0, y as f32 * 96.0).into(),
                (96.0, 96.0).into(),
                (((y * glyph_count.0) + x) as f32, 0.0, 0.0).into()
            ));
        }
    }

    QuadShapeVertexPrimitiveRenderer::new(gl, shader_program, points, glyph_count.1 * glyph_count.0)
}