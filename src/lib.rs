#![allow(unused_imports)]
extern crate gl;
extern crate glutin;

macro_rules! log_draw {
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "log_draw")]
            println!($($arg)*);
        }
    };
}

macro_rules! check_gl_errors {
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "log_draw")] {
                let error = unsafe { gl::GetError() };

                match error {
                    gl::INVALID_ENUM => {
                        println!("gl::glGetError() -> gl::INVALID_ENUM");
                        println!("  -> An unacceptable value is specified for an enumerated argument.");
                        println!("  -> The offending command is ignored and has no other side effect");
                        println!("  -> than to set the error flag.");
                    },
                    gl::INVALID_VALUE => {
                        println!("gl::glGetError() -> gl::INVALID_VALUE");
                        println!("  -> A numeric argument is out of range. The offending command is");
                        println!("  -> ignored and has no other side effect than to set the error flag.");
                    },
                    gl::INVALID_OPERATION => {
                        println!("gl::glGetError() -> gl::INVALID_OPERATION");
                        println!("  -> The specified operation is not allowed in the current state.");
                        println!("  -> The offending command is ignored and has no other side effect");
                        println!("  -> than to set the error flag.");
                    },
                    gl::INVALID_FRAMEBUFFER_OPERATION => {
                        println!("gl::glGetError() -> gl::INVALID_FRAMEBUFFER_OPERATION");
                        println!("  -> The command is trying to render to or read from the framebuffer");
                        println!("  -> while the currently bound framebuffer is not framebuffer");
                        println!("  -> complete (i.e. the return value from  glCheckFramebufferStatus");
                        println!("  -> is not GeeL_FRAMEBUFFER_COMPLETE). The offending command is");
                        println!("  -> ignored and has no other side effect than to set the error flag.");
                    },
                    gl::OUT_OF_MEMORY => {
                        println!("gl::glGetError() -> gl::OUT_OF_MEMORY");
                        println!("  -> There is not enough memory left to execute the command. The");
                        println!("  -> state of the GL is undefined, except for the state of the");
                        println!("  -> error flags, after this error is recorded.");
                    },
                    gl::NO_ERROR => { println!("No error"); },
                    _ => {}
                };
            }
        }
    };
}

pub mod draw_builder;
pub mod gl_helpers;
pub mod rugl;
pub mod buffers;
pub mod uniforms;
mod primitive;
pub use primitive::Primitive;
pub use rugl::*;
