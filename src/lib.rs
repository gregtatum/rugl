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

pub mod draw_builder;
pub mod gl_helpers;
pub mod rugl;
pub mod buffers;
pub mod uniforms;
mod primitive;
pub use primitive::Primitive;
pub use rugl::*;
