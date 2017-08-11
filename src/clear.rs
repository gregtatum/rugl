use super::gl::types::*;
use super::gl;

pub struct Clear {
    pub color: Option<[f32; 4]>,
    pub depth: Option<f64>,
    pub stencil: Option<i32>
}

impl Clear {
    pub fn new() -> Clear {
        Clear {
            color: None,
            depth: None,
            stencil: None
        }
    }

    pub fn execute(&self) {
        unsafe {
            let mut clear_bits: GLenum = 0;
            match self.color {
                Some(color) => {
                    clear_bits = clear_bits | gl::COLOR_BUFFER_BIT;
                    gl::ClearColor(color[0], color[1], color[2], color[3]);
                    log_draw!("gl::ClearColor({}, {}, {}, {})", color[0], color[1], color[2], color[3]);
                },
                None => {},
            };
            match self.depth {
                Some(depth) => {
                    clear_bits = clear_bits | gl::DEPTH_BUFFER_BIT;
                    gl::ClearDepth(depth);
                    log_draw!("gl::ClearDepth({})", depth);
                }
                None => {}
            };
            match self.stencil {
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

    pub fn get_execute_lambda(self) -> Box<Fn()> {
        Box::new(move || self.execute())
    }
}

#[macro_export]
macro_rules! rugl_clear {
    ($($key:ident => $value:expr),*) => {
        {
            let mut clear = $crate::clear::Clear::new();
            $( rugl_clear_key_pair!(clear, $key => $value); )*
            clear.get_execute_lambda()
        }
    };
}

#[macro_export]
macro_rules! rugl_clear_key_pair {
    ($clear:expr, color => $value:expr) => {
        match &mut $clear {
            clear => clear.color = Some($value)
        };
    };
    ($clear:expr, depth => $value:expr) => {
        match &mut $clear {
            clear => clear.depth = Some($value)
        };
    };
    ($clear:expr, stencil => $value:expr) => {
        match &mut $clear {
            clear => clear.stencil = Some($value)
        };
    };
}
