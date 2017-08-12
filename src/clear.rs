use super::gl::types::*;
use super::gl;
use super::rugl;

#[macro_export]
macro_rules! rugl {
    ($rugl:ident.$method:ident, { $($key:ident => $value:expr),* }) => {
        {
            let mut tmp_struct = $rugl.$method();
            $( tmp_struct.$key = Some($value); )*
            tmp_struct.make_execute_fn()
        }
    };
}

/// `Clear` combines `glClearColor`, `glClearDepth`, `glClearStencil` and `glClear` into a
/// single procedure, which has the following default usage:
///
///     #[macro_use]
///     extern crate rugl;
///
///     fn main() {
///         let rugl = rugl::init_headless();
///
///         // Create a clear command that clears the color, depth and stencil.
///         let clear_all = rugl!(rugl.clear, {
///             color => [0.0, 0.0, 0.0, 1.0],
///             depth => 1.0,
///             stencil => 0
///         });
///
///         // Create a clear command that clears only the color.
///         let clear_only_color = rugl!(rugl.clear, {
///             color => [0.0, 0.0, 0.0, 1.0]
///         });
///     }
///
/// Then run `clear_all()` or `clear_only_color()` to run the command. The `rugl!` macro quickly
/// sets the values and returns a closure that can perform the action. While this is the preferred
/// API, the `Clear` struct can be used by itself, especially if the clear command needs to be
/// mutated over time.
///
///     use rugl;
///     let rugl = rugl::init_headless();
///
///     let mut clear = rugl.clear();
///     clear.color = Some([0.0, 0.0, 0.0, 1.0]);
///     clear.execute();
///
///     // The returned function assumes ownership of the Clear struct.
///     let clear_black = clear.make_execute_fn();
///     clear_black();
///
pub struct Clear {
    /// Sets the clear color
    pub color: Option<[f32; 4]>,
    /// Sets the clear depth value
    pub depth: Option<f64>,
    /// Sets the clear stencil value
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
    pub fn make_execute_fn(self) -> Box<Fn()> {
        Box::new(move || self.execute())
    }
}

#[cfg(test)]
mod tests {
    use super::rugl::init_headless;
    #[test]
    fn clear_on_a_macro() {
        let rugl = init_headless();

        #[allow(unused_variables)]
        let clear1 = rugl!(rugl.clear, {
            color => [0.3, 0.2, 0.3, 1.0],
            depth => 1.0,
            stencil => 0
        });

        #[allow(unused_variables)]
        let clear2 = rugl!(rugl.clear, {
            color => [0.3, 0.2, 0.3, 1.0],
            stencil => 0
        });

        #[allow(unused_variables)]
        let clear2 = rugl!(rugl.clear, {
            color => [0.3, 0.2, 0.3, 1.0]
        });
    }

    #[test]
    fn clear_by_object() {
        let rugl = init_headless();

        #[allow(unused_variables)]
        let clear = {
            let mut clear = rugl.clear();
            clear.color = Some([0.3, 0.2, 0.3, 1.0]);
            clear.depth = Some(1.0);
            clear.stencil = Some(0);
            clear.make_execute_fn()
        };
        // clear();
    }

}
