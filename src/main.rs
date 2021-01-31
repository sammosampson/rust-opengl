#[macro_use] extern crate failure;
pub mod examples;
pub mod error_handling;
pub mod resources;
pub mod window;
pub mod render_gl;


fn main() {
    if let Err(e) = run() {
        println!("{}", error_handling::failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {
    //examples::vertex_coloured_triangle::run()?;
    //examples::vertex_coloured_quad::run()?;
    //examples::circle_on_quad::run()?;
    //examples::vertex_textured_quad::run()?;
    //examples::texture_coordinate_driven_circle_on_quad::run()?;
    //examples::points_driven_circle_on_quad::run()?;
    //examples::vertex_coloured_triangle_with_geom_shader::run()?;
    //examples::points_driven_rectangle_on_quad::run()?;
    //examples::points_driven_text_on_quad::run()?;
    //examples::updateable_quads::run()?;
    examples::full_renderer::run()?;
    //examples::convert_font::run()?;
    Ok(())
}