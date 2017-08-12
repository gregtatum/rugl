use super::gl::types::*;
use super::gl;

/// `rugl_clear!` combines `glClearColor`, `glClearDepth`, `glClearStencil` and `glClear` into a
/// single procedure, which has the following usage:
///
///     let clear = rugl_clear!(
///         color => [0.0, 0.0, 0.0, 1.0],
///         depth: 1.0,
///         stencil: 0
///     );
///
///     clear();
///
/// If an option is not present, then the corresponding buffer is not cleared
///
/// | Property  | Type       | Description                  |
/// | --------- | ---------- | ---------------------------- |
/// | `color`   | `[f32; 4]` | Sets the clear color         |
/// | `depth`   | `f64`      | Sets the clear depth value   |
/// | `stencil` | `i32`      | Sets the clear stencil value |
///
/// See [rugl::Clear](./clear/index.html) for the backing implementation.

#[macro_export]
macro_rules! rugl_clear {
    ($($key:ident => $value:expr),*) => {
        {
            let mut clear = $crate::clear::Clear::new();
            $( clear.$key = Some($value); )*
            clear.get_execute_lambda()
        }
    };
}

/// The backing struct for the `rugl_clear!` macro. It can be used as a mutable struct
/// if needed, although the macro is the preferred usage.
///
///     let mut clear = Clear::new();
///     clear.color = Some([0.0, 0.0, 0.0, 1.0]);
///     clear.execute();
///
///     let clearBlack = clear.get_execute_lambda();
///     clearBlack();
///
pub struct Clear {
    /// Optionally sets the clear color
    pub color: Option<[f32; 4]>,
    /// Optionally sets the clear depth value
    pub depth: Option<f64>,
    /// Optionally sets the clear stencil value
    pub stencil: Option<i32>
}

impl Clear {
    /// Create a new clear object.
    pub fn new() -> Clear {
        Clear {
            color: None,
            depth: None,
            stencil: None
        }
    }

    /// Execute the glClear with the set values.
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

    /// Consume the struct and get a closure over `execute()`.
    pub fn get_execute_lambda(self) -> Box<Fn()> {
        Box::new(move || self.execute())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn clear_on_a_macro() {
        let clear1 = rugl_clear!(
            color => [0.3, 0.2, 0.3, 1.0],
            depth => 1.0,
            stencil => 2
        );

        let clear2 = rugl_clear!(
            color => [0.3, 0.2, 0.3, 1.0],
            stencil => 2
        );

        let clear3 = rugl_clear!(
            color => [0.3, 0.2, 0.3, 1.0]
        );
    }
}
