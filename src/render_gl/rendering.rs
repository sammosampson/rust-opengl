use gl;
use failure;
use crate::render_gl;

pub struct Blender {
}

impl Blender {
    pub fn new() -> Blender {
        Blender {
        }
    }

    pub fn set_used(&self, gl: &gl::Gl) {
        unsafe {
            gl.Enable(gl::BLEND);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn set_unused(&self, gl: &gl::Gl) {
        unsafe {
            gl.Disable(gl::BLEND);
        }
    }
}

pub trait RenderDrawType {
    const DRAW_TYPE: gl::types::GLuint;
}

pub struct TriangleStripRenderDrawType;
impl RenderDrawType for TriangleStripRenderDrawType {
    const DRAW_TYPE: gl::types::GLuint = gl::TRIANGLE_STRIP;
}

pub struct PointsRenderDrawType;
impl RenderDrawType for PointsRenderDrawType {
    const DRAW_TYPE: gl::types::GLuint = gl::POINTS;
}

pub type VertexColouredVertexPrimitiveRenderer = PrimitiveRenderer::<render_gl::vertices::VertexColouredVertex, TriangleStripRenderDrawType>;
pub type TexturedVertexPrimitiveRenderer = PrimitiveRenderer::<render_gl::vertices::TexturedVertex, TriangleStripRenderDrawType>;
pub type QuadShapeVertexPrimitiveRenderer = PrimitiveRenderer::<render_gl::vertices::QuadShapeVertex, PointsRenderDrawType>;
pub type RenderPrimitivePrimitiveRenderer = PrimitiveRenderer::<render_gl::vertices::RenderPrimitive, PointsRenderDrawType>;

pub struct PrimitiveRenderer<TVertex, TDrawtype> 
    where TVertex : render_gl::vertices::Vertex, TDrawtype : RenderDrawType {
    program: render_gl::Program,
    vbo: render_gl::buffer::ArrayBuffer,
    vao: render_gl::buffer::VertexArray,
    indices: i32,
    _vertex_marker: ::std::marker::PhantomData<TVertex>,
    _draw_type_marker: ::std::marker::PhantomData<TDrawtype>
} 

impl<TVertex, TDrawType> PrimitiveRenderer<TVertex, TDrawType>
    where TVertex : render_gl::vertices::Vertex, TDrawType : RenderDrawType {
    pub fn new(
        gl: &gl::Gl,
        program: render_gl::Program,
        vertices: Vec<TVertex>,
        indices: i32) -> Result<PrimitiveRenderer<TVertex, TDrawType> , failure::Error>
    {    
        let vbo = render_gl::buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.set_draw_data(&vertices);
        vbo.unbind();

        let vao = render_gl::buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        TVertex::set_gl_attribute_pointers(gl);
        vbo.unbind();
        vao.unbind();

        Ok(Self {
            program,
            vbo,
            vao,
            indices,
            _vertex_marker: ::std::marker::PhantomData,
            _draw_type_marker: ::std::marker::PhantomData,
        })
    }

    pub fn update_vertices(&self, vertices: Vec<TVertex>, offset: usize) {
        self.vbo.bind();
        self.vbo.update_draw_data(&vertices, offset);
        self.vbo.unbind();
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.render_portion(gl, 0, self.indices);
    }

    pub fn render_portion(&self, gl: &gl::Gl, offset: i32, count: i32) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                TDrawType::DRAW_TYPE,
                offset,
                count
            );
        }
    }

    pub fn program(&self) -> &render_gl::Program {
        &self.program
    }
}