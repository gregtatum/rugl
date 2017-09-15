/// The preferred API for generating commands is to use the `rugl!` macro. This is a shorthand
/// interface to declare descriptions of various commands.
#[macro_export]
macro_rules! rugl {
    ($rugl:ident.draw, { $($key:ident => $value:tt),* }) => {
        {
            rugl_draw!($rugl, {
                $($key => $value),*
            })
        }
    };
    ($rugl:ident.$method:ident, { $($key:ident => $value:expr),* }) => {
        {
            let mut tmp_struct = $rugl.$method();
            $( tmp_struct.$key = Some($value); )*
            tmp_struct.make_execute_fn()
        }
    };
}

/// Internal macro for `rugl!(rugl.draw, { ... })`
#[macro_export]
macro_rules! rugl_draw {
    // Take an instance of rugl::init, and { key => value }*.
    ($rugl:expr, { $($key:ident => $value:tt),* }) => {
        {
            #![allow(unused_mut)]
            let mut draw = $rugl.draw();
            // Go through the rest of the key/pair values and set them now.
            $( rugl_draw_key_pair!($key, draw, $value); )*
            draw.finalize()
        }
    };
}

/// Internal macro for `rugl!(rugl.draw, { ... })`
#[macro_export]
macro_rules! rugl_draw_key_pair {
    // Match on attributes, go through each attribute and add them.
    (attributes, $draw:expr, { $($key:ident => $value:expr),* }) => {
        $( $draw.add_attribute(stringify!($key), $value); )*
    };

    // Match on uniforms, go through each uniform and add them.
    (uniforms, $draw:expr, { $($key:ident => $value:expr),* }) => {
        $( $draw.add_uniform(stringify!($key), Box::new($value)); )*
    };


    // Catch some typos to avoid confusing errors.
    (uniform, $draw:expr, $value:tt) => {
        panic!("The word \"uniform\" was used instead of \"uniforms\" when creating a draw call.");
    };
    (attribute, $draw:expr, $value:tt) => {
        panic!("The word \"attribute\" was used instead of \"attributes\" when creating a draw call.");
    };

    // Setters
    (elements, $draw:expr, $value:expr) => { $draw.set_elements($value); };
    (primitive, $draw:expr, $value:expr) => { $draw.set_primitive($value); };
    (count, $draw:expr, $value:expr) => { $draw.count = $value; };
    ($key:ident, $draw:expr, $value:expr) => { $draw.$key = Some($value); };
}

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
                    gl::NO_ERROR => { println!("gl::glGetError() -> gl::NO_ERROR"); },
                    _ => {}
                };
            }
        }
    };
}
