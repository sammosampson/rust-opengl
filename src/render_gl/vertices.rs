
use crate::render_gl::data;

pub trait Vertex {
    fn set_gl_attribute_pointers(gl: &::gl::Gl);
}
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct RenderPrimitive {
    pos: data::u16_u16,
    dimensions: data::u16_u16,
    inner_colour: data::f32_f32_f32_f32,
    outer_colour: data::f32_f32_f32_f32,
    identification: data::u16_u16,
    extra_data_1: data::f32_f32_f32_f32,
    extra_data_2: data::f32_f32_f32_f32
}

impl RenderPrimitive {
    pub fn with_position_size_colours_identification_and_data(
        position: data::u16_u16,
        dimensions: data::u16_u16,
        inner_colour: data::f32_f32_f32_f32,
        outer_colour: data::f32_f32_f32_f32,
        identification: data::u16_u16,
        extra_data_1: data::f32_f32_f32_f32,
        extra_data_2: data::f32_f32_f32_f32) -> Self {
        Self {
            pos: position,
            dimensions,
            inner_colour,
            outer_colour,
            identification,
            extra_data_1,
            extra_data_2
        }
    }

    pub fn circle(
        position: (u16, u16),
        radius: u16,
        inner_colour: (f32, f32, f32, f32),
        outer_colour: (f32, f32, f32, f32),
        stroke_width: f32) -> Self {
        RenderPrimitive::with_position_size_colours_identification_and_data(
            position.into(),
            (radius, radius).into(),
            inner_colour.into(),
            outer_colour.into(),
            (0, 0).into(),
            (stroke_width, 0.0, 0.0, 0.0).into(),
            (0.0, 0.0, 0.0, 0.0).into()
        ) 
    }

    pub fn rectangle(
        position: (u16, u16),
        dimensions: (u16, u16),
        inner_colour: (f32, f32, f32, f32),
        outer_colour: (f32, f32, f32, f32),
        stroke_width: f32,
        corner_radii: (f32, f32, f32, f32)) -> Self {
        return RenderPrimitive::with_position_size_colours_identification_and_data(
            position.into(),
            dimensions.into(),
            inner_colour.into(),
            outer_colour.into(),
            (1, 0).into(),
            (stroke_width, 0.0, 0.0, 0.0).into(),
            corner_radii.into()
        )
    }

    pub fn text(
        position: (u16, u16),
        dimensions: (u16, u16),
        colour: (f32, f32, f32, f32),
        glyph_index: u16) -> Self {
        return RenderPrimitive::with_position_size_colours_identification_and_data(
            position.into(),
            dimensions.into(),
            colour.into(),
            colour.into(),
            (2, glyph_index).into(),
            (0.0, 0.0, 0.0, 0.0).into(),
            (0.0, 0.0, 0.0, 0.0).into()
        )
    }
}

impl Vertex for RenderPrimitive {
    fn set_gl_attribute_pointers(gl: &::gl::Gl) {
        let stride = ::std::mem::size_of::<RenderPrimitive>();

        let offset = 0;

        let location = 0;
        data::u16_u16::set_gl_attribute_pointer(gl, stride, location, offset);
        let offset = offset + ::std::mem::size_of::<data::u16_u16>();

        let location = 1;
        data::u16_u16::set_gl_attribute_pointer(gl, stride, location, offset);
        let offset = offset + ::std::mem::size_of::<data::u16_u16>();

        let location = 2;
        data::f32_f32_f32_f32::set_gl_attribute_pointer(gl, stride, location, offset);
        let offset = offset + ::std::mem::size_of::<data::f32_f32_f32_f32>();

        let location = 3;
        data::f32_f32_f32_f32::set_gl_attribute_pointer(gl, stride, location, offset);
        let offset = offset + ::std::mem::size_of::<data::f32_f32_f32_f32>();

        let location = 4;
        data::u16_u16::set_gl_attribute_pointer(gl, stride, location, offset);
        let offset = offset + ::std::mem::size_of::<data::u16_u16>();

        let location = 5;
        data::f32_f32_f32_f32::set_gl_attribute_pointer(gl, stride, location, offset);
        let offset = offset + ::std::mem::size_of::<data::f32_f32_f32_f32>();

        let location = 6;
        data::f32_f32_f32_f32::set_gl_attribute_pointer(gl, stride, location, offset);
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct QuadShapeVertex {
    pos: data::f32_f32,
    dimensions: data::f32_f32,
    extra_data: data::f32_f32_f32
}

impl QuadShapeVertex {
    pub fn with_position_and_size(
        position: data::f32_f32,
        dimensions: data::f32_f32,
        extra_data: data::f32_f32_f32) -> Self {
        Self {
            pos: position,
            dimensions,
            extra_data
        }
    }
}

impl Vertex for QuadShapeVertex {
    fn set_gl_attribute_pointers(gl: &::gl::Gl) {
        let stride = ::std::mem::size_of::<QuadShapeVertex>();

        let offset = 0;
        let location = 0;
        data::f32_f32::set_gl_attribute_pointer(gl, stride, location, offset);

        let offset = offset + ::std::mem::size_of::<data::f32_f32>();
        let location = 1;
        data::f32_f32::set_gl_attribute_pointer(gl, stride, location, offset);

        let offset = offset + ::std::mem::size_of::<data::f32_f32>();
        let location = 2;
        data::f32_f32_f32::set_gl_attribute_pointer(gl, stride, location, offset);
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertexColouredVertex {
    pos: data::f32_f32_f32,
    clr: data::u2_u10_u10_u10_rev_float
}

impl VertexColouredVertex {
    pub fn with_position_and_colour(
        position: data::f32_f32_f32,
        colour: data::u2_u10_u10_u10_rev_float) -> Self {
        Self {
            pos: position,
            clr: colour
        }
    }
}

impl Vertex for VertexColouredVertex {
    fn set_gl_attribute_pointers(gl: &::gl::Gl) {
        let stride = ::std::mem::size_of::<VertexColouredVertex>();

        let offset = 0;
        let location = 0;
        data::f32_f32_f32::set_gl_attribute_pointer(gl, stride, location, offset);

        let offset = offset + ::std::mem::size_of::<data::f32_f32_f32>();
        let location = 1;
        data::u2_u10_u10_u10_rev_float::set_gl_attribute_pointer(gl, stride, location, offset);
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct TexturedVertex {
    pos: data::f32_f32_f32,
    uv: data::f32_f32
}

impl TexturedVertex {
    pub fn with_position_and_texture_coordinate(
        position: data::f32_f32_f32,
        uv: data::f32_f32) -> Self {
        Self {
            pos: position,
            uv
        }
    }
}

impl Vertex for TexturedVertex {
    fn set_gl_attribute_pointers(gl: &::gl::Gl) {
        let stride = ::std::mem::size_of::<TexturedVertex>();

        let offset = 0;
        let location = 0;
        data::f32_f32_f32::set_gl_attribute_pointer(gl, stride, location, offset);

        let offset = offset + ::std::mem::size_of::<data::f32_f32_f32>();
        let location = 1;
        data::f32_f32::set_gl_attribute_pointer(gl, stride, location, offset);
    }
}