//! # Declarative Stateless OpenGL
//!
//! rugl is a port of [regl](http://regl.party) (a WebGL library). It is currently in development.
//! as a side project. The goal is provide a declarative description of an OpenGL draw call,
//! including the shader source, attributes, uniforms, etc, and generate a lambda that can then
//! be used as a function with the only side effect being something rendered to the screen.
//! The draw calls will eventually have an internal state management for the gl state and will
//! execute the minimal set of state changes between two different draw calls.
//!
//! # Example draw call
//!
//! ```
//! # #[macro_use] extern crate rugl;
//! # fn main() {
//! // let mut rugl = rugl::init_headless();
//! //
//! // let draw_single_triangle = rugl!(rugl.draw, {
//! //     vert => "
//! //         #version 150
//! //         in vec2 position;
//! //         void main() {
//! //             gl_Position = vec4(position, 0.0, 1.0);
//! //         }
//! //     ",
//! //     frag => "
//! //         #version 150
//! //         out vec4 out_color;
//! //         void main() {
//! //             out_color = vec4(1.0, 1.0, 1.0, 1.0);
//! //         }
//! //     ",
//! //     attributes => {
//! //         position => {&vec![
//! //              0.0f32,  0.5,
//! //              0.5, -0.5,
//! //             -0.5, -0.5
//! //         ]}
//! //     },
//! //     count => 3
//! // });
//! //
//! // let clear = rugl!(rugl.clear, {
//! //     color => [0.3, 0.2, 0.3, 1.0],
//! //     depth => 1.0
//! // });
//! //
//! // rugl.frame(|env| {
//! //     clear();
//! //     draw_single_triangle(env);
//! // });
//! # }
//! ```

#![allow(unused_imports)]
extern crate gl;
extern crate glutin;


#[macro_use]
pub mod macros;
pub mod draw;
pub mod clear;
pub mod gl_helpers;
pub mod rugl;
pub mod buffers;
pub mod uniforms;
mod primitive;
pub use primitive::Primitive;
pub use rugl::init;
pub use rugl::init_headless;
