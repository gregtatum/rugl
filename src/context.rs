use super::gl;
use super::gl::types::*;

pub trait Context {
    fn clear_color(&self, [f32; 4]);
    fn clear_depth(&self, f64);
    fn clear_stencil(&self, i32);
    fn clear(&self, GLenum);
}

pub struct LiveContext {}
pub struct HeadlessContext {}

impl Context for LiveContext {
    fn clear_color(&self, color: [f32; 4]) {
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], color[3]);
        };
        log_draw!(
            "gl::ClearColor({}, {}, {}, {})",
            color[0],
            color[1],
            color[2],
            color[3]
        );
    }

    fn clear_depth(&self, depth: f64) {
        unsafe {
            gl::ClearDepth(depth);
        };
        log_draw!("gl::ClearDepth({})", depth);
    }

    fn clear_stencil(&self, stencil: i32) {
        unsafe {
            gl::ClearStencil(stencil);
        };
        log_draw!("gl::ClearStencil({})", stencil);
    }

    fn clear(&self, bits: GLenum) {
        unsafe {
            gl::Clear(bits);
        };
        log_draw!("gl::Clear({})", bits);
    }
}

impl Context for HeadlessContext {
    fn clear_color(&self, _: [f32; 4]) {}
    fn clear_depth(&self, _: f64) {}
    fn clear_stencil(&self, _: i32) {}
    fn clear(&self, _: GLenum) {}
}
