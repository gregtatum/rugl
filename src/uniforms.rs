use super::gl;
use super::gl::types::*;

pub trait UniformValue {
    fn set_uniform(&self, GLint);
}

impl UniformValue for f32 {
    fn set_uniform(&self, location: GLint) {
        unsafe {
            #[cfg(feature = "log_draw")]
            println!("gl::Uniform1f({}, {}))", location, self);
            gl::Uniform1f(location, self.clone());
        }
    }
}
