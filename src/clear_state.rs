use std::collections::HashMap;
use super::gl;
use super::gl::types::*;

pub enum ClearChanged {
    Clear,
}

pub struct ClearCommand {
    pub stencil: Option<i32>,
    pub depth: Option<f64>,
    pub color: Option<[f32; 4]>,
}

pub fn diff_clear (previous: &ClearState, next: &ClearState) {
    unsafe {
        let mut clear_bits: GLenum = 0;
        match next.color {
            Some(color) => {
                clear_bits = clear_bits | gl::COLOR_BUFFER_BIT;
                gl::ClearColor(color[0], color[1], color[2], color[3]);
                log_draw!(
                    "gl::ClearColor({}, {}, {}, {})",
                    color[0],
                    color[1],
                    color[2],
                    color[3]
                );
            }
            None => {}
        };
        match next.depth {
            Some(depth) => {
                clear_bits = clear_bits | gl::DEPTH_BUFFER_BIT;
                gl::ClearDepth(depth);
                log_draw!("gl::ClearDepth({})", depth);
            }
            None => {}
        };
        match next.stencil {
            Some(stencil) => {
                clear_bits = clear_bits | gl::STENCIL_BUFFER_BIT;
                gl::ClearStencil(stencil);
                log_draw!("gl::ClearStencil({})", depth);
            }
            None => {}
        };
        if clear_bits != 0 {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

}
