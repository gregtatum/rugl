#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

use super::gl_helpers;
use super::glutin;
use super::buffers::BufferableData;
use super::gl::types::*;
use super::gl;
use std::ffi::CString;
use std::collections::HashMap;
use std::string;

pub struct Environment {
    pub window: glutin::Window
}

pub struct Rugl {
    environment: Environment
}

pub fn init() -> Rugl {
    let window = glutin::Window::new().unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    unsafe { window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    Rugl {
        environment: Environment {
            window: window
        }
    }
}

impl Rugl {
    pub fn draw(&self) -> DrawBuilder {
        DrawBuilder::new(&self.environment)
    }

    pub fn frame<F>(&self, callback: F) where
        F: Fn()
    {
        for event in self.environment.window.wait_events() {
            unsafe {
                gl::ClearColor(0.3, 0.2, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            callback();

            self.environment.window.swap_buffers().unwrap();

            if let glutin::Event::Closed = event {
                break;
            }
        }
    }
}

pub struct DrawConfig<'env> {
    pub environment: &'env Environment,
    pub vert: Option<&'static str>,
    pub frag: Option<&'static str>,
    pub attributes: Vec<(String, GLuint)>,
    pub count: i32
}

pub struct DrawBuilder<'env> {
    pub config: DrawConfig<'env>
}

impl<'env> DrawBuilder<'env> {
    pub fn new<'a>(environment: &'a Environment) -> DrawBuilder {
        DrawBuilder {
            config: DrawConfig {
                environment: environment,
                vert: None,
                frag: None,
                attributes: Vec::new(),
                count: 0
            }
        }
    }

    pub fn vert(mut self, source: &'static str) -> DrawBuilder<'env> {
        self.config.vert = Some(source);
        self
    }

    pub fn frag(mut self, source: &'static str) -> DrawBuilder<'env> {
        self.config.frag = Some(source);
        self
    }

    pub fn attribute(
        mut self, name: &str, vertices: &BufferableData
    ) -> DrawBuilder<'env> {
        // gl_helpers::log_draw!("Adding attribute {:?}", name);
        self.config.attributes.push(
            (name.to_string(), vertices.to_buffer())
        );
        self
    }

    pub fn count(mut self, count: i32) -> DrawBuilder<'env> {
        self.config.count = count;
        self
    }

    pub fn finalize(self) -> Box<Fn()> {
        let config = self.config;
        let vertex_shader = match config.vert {
            Some(vert) => Some(gl_helpers::compile_shader(vert, gl::VERTEX_SHADER)),
            None => None
        };
        let fragment_shader = match config.frag {
            Some(frag) => Some(gl_helpers::compile_shader(frag, gl::FRAGMENT_SHADER)),
            None => None
        };

        let program = match (vertex_shader, fragment_shader) {
            (Some(vs), Some(fs)) => Some(gl_helpers::link_program(&vs, &fs)),
            _ => None
        };

        let programs_attributes = match program {
            Some(program) => gl_helpers::get_program_attributes(program),
            _ => Vec::new()
        };

        let buffers: Vec<Option<GLuint>> = programs_attributes.iter().map(|attribute_info| {
            let attribute_config = config.attributes
                .iter()
                .find(|attribute_tuple| {
                    attribute_tuple.0 == attribute_info.name
                });

            match attribute_config {
                Some(&(_, ref buffer)) => { Some(*buffer) },
                _ => None
            }
        }).collect();

        let count = config.count;
        let vao = match program {
            Some(_) => {
                // Create a vertex array object that stores all of the attributes and buffer
                // information.
                let vao = gl_helpers::create_vao();
                gl_helpers::bind_vao(vao);

                // Go through each attribute, and bind it to the proper slot with the
                // proper shapes.
                for i in 0..programs_attributes.len() {
                    let attribute_info = programs_attributes.get(i).unwrap();
                    let buffer = buffers[i];
                    match buffer {
                        Some(vbo) => gl_helpers::bind_attribute_buffer(vbo, attribute_info),
                        None => {}
                    }
                }
                // Un-bind the vao, now when we bind it again, it will restore the state
                // of our shader.
                gl_helpers::bind_vao(0);
                Some(vao)
            },
            None => None
        };

        return Box::new(move || {
            #[cfg(feature = "debug_draw")]
            println!("----------------------------------------------------");

            match program {
                Some(program) => {
                    gl_helpers::use_program(program);
                    gl_helpers::bind_vao(vao.unwrap());
                    gl_helpers::draw_arrays(gl::TRIANGLES, 0, count);
                },
                None => {}
            };
        })
    }
}
