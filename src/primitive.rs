use super::gl;
use super::gl::types::*;

pub enum Primitive {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan
}

impl Primitive {
    pub fn to_gl_enum(&self) -> GLenum {
        match self {
            &Primitive::Points => gl::POINTS,
            &Primitive::Lines => gl::LINES,
            &Primitive::LineStrip => gl::LINE_STRIP,
            &Primitive::LineLoop => gl::LINE_LOOP,
            &Primitive::Triangles => gl::TRIANGLES,
            &Primitive::TriangleStrip => gl::TRIANGLE_STRIP,
            &Primitive::TriangleFan => gl::TRIANGLE_FAN
        }
    }
}
