use crate::render_gl::{
    rendering:: {
        Blender,
        RenderPrimitivePrimitiveRenderer
    }, 
    vertices::RenderPrimitive,
    viewport::Viewport,
    buffer::{
        ColorBuffer
    },
    font::{
        FontSampler
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
        .with_title("Full renderer")
        .with_width(900)
        .with_height(700)
        .build()?;

    let gl = gl::Gl
        ::load_with(|process_name|window_manager.get_process_address(process_name));

    let res = resources::Resources
        ::from_relative_exe_path(std::path::Path::new("assets-07"))?;
    
    let shader_program = Program
        ::from_res(&gl, &res, &[
            "shaders/third/full_render.vert",
            "shaders/third/full_render.geom",
            "shaders/third/full_render.frag"])?;

    let white = (1.0, 1.0, 1.0, 1.0);        
    let black = (0.0, 0.0, 0.0, 1.0);        
    let points: Vec<RenderPrimitive> = vec![
        RenderPrimitive::circle((100, 100), 100, white, black, 5.0),
        RenderPrimitive::rectangle((1000, 1000), (600, 600), white, black, 20.0, (0.1, 0.3, 0.4, 0.2)),
        RenderPrimitive::text((200, 200), (600, 600), white, 37),
    ];

    let renderer = RenderPrimitivePrimitiveRenderer::new(&gl, shader_program, points, 3)?;

    renderer.program().set_used();
    renderer.program().set_uniform_vec2f("uResolution", 1800.0, 1400.0);

    let mut sampler = FontSampler::from_font_resource(&gl, &res, "fonts/segoeui-1.png", 500, (96, 96), 0)?;
    sampler.send_to_shader_property(&renderer.program(), "font_buffer");
   
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
    
    window_manager.loop_until_window_quit(|_total_time, _frame_time| {
        //let total_time_enlarged = total_time * 8.0;
    })?;
    Ok(())
}