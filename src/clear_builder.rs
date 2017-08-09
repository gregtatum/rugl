use super::gl::types::*;
use super::gl;

pub struct Clear {
    color_val: Option<[f32; 4]>,
    depth_val: Option<f64>,
    stencil_val: Option<i32>
}

impl Clear {
    pub fn new() -> Clear {
        Clear {
            color_val: None,
            depth_val: None,
            stencil_val: None
        }
    }

    pub fn color(mut self, color: [f32; 4]) -> Clear {
        self.color_val = Some(color);
        self
    }

    pub fn depth(mut self, depth: f64) -> Clear {
        self.depth_val = Some(depth);
        self
    }

    pub fn stencil(mut self, stencil: i32) -> Clear {
        self.stencil_val = Some(stencil);
        self
    }

    pub fn execute(&self) {
        unsafe {
            let mut clear_bits: GLenum = 0;
            match self.color_val {
                Some(color) => {
                    clear_bits = clear_bits | gl::COLOR_BUFFER_BIT;
                    gl::ClearColor(color[0], color[1], color[2], color[3]);
                    log_draw!("gl::ClearColor({}, {}, {}, {})", color[0], color[1], color[2], color[3]);
                },
                None => {},
            };
            match self.depth_val {
                Some(depth) => {
                    clear_bits = clear_bits | gl::DEPTH_BUFFER_BIT;
                    gl::ClearDepth(depth);
                    log_draw!("gl::ClearDepth({})", depth);
                }
                None => {}
            };
            match self.stencil_val {
                Some(stencil) => {
                    clear_bits = clear_bits | gl::STENCIL_BUFFER_BIT;
                    gl::ClearStencil(stencil);
                    log_draw!("gl::ClearStencil({})", depth);
                },
                None => {}
            };
            if clear_bits != 0 {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
        }
    }
}
