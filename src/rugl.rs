#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

use super::gl_helpers;
use super::glutin;
use super::gl::types::*;
use super::gl;
use std::ffi::CString;
use std::collections::HashMap;
use std::string;

pub struct RuglConfig {
    pub vert: Option<&'static str>,
    pub frag: Option<&'static str>,
    pub attributes: Vec<(String, Vec<GLfloat>)>
}

pub struct Rugl {
    pub config: RuglConfig
}

impl Rugl {
    pub fn new() -> Rugl {
        Rugl {
            config: RuglConfig {
                vert: None,
                frag: None,
                attributes: Vec::new()
            }
        }
    }

    pub fn vert(mut self, source: &'static str) -> Rugl {
        self.config.vert = Some(source);
        self
    }

    pub fn frag(mut self, source: &'static str) -> Rugl {
        self.config.frag = Some(source);
        self
    }

    pub fn attribute(mut self, name: &str, vertices: Vec<GLfloat>) -> Rugl {
        self.config.attributes.push(
            (name.to_string(), vertices)
        );
        self
    }

    pub fn finalize(self) -> Box<Fn()> {
        let config = self.config;
        let vertex_shader = match config.vert {
            Some(vert) => Some(unsafe { gl_helpers::compile_shader(vert, gl::VERTEX_SHADER) }),
            None => None
        };
        let fragment_shader = match config.frag {
            Some(frag) => Some(unsafe { gl_helpers::compile_shader(frag, gl::FRAGMENT_SHADER) }),
            None => None
        };

        let program = match (vertex_shader, fragment_shader) {
            (Some(fs), Some(vs)) => Some(unsafe { gl_helpers::link_program(&vs, &fs) }),
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
                Some(&(_, ref values)) => { Some(unsafe { gl_helpers::create_buffer(&values) }) },
                _ => None
            }
        }).collect();

        return Box::new(move || {
            // TODO - This hashmap for looking up buffers is a mess. We need
            // to find a better way to do this.

            for i in 0..programs_attributes.len() {
                let attribute_info = programs_attributes.get(i).unwrap();
                let buffer = buffers[i];
                match buffer {
                    Some(vbo) => unsafe {
                        println!("Binding the attribute {:?}", attribute_info.name);
                        gl_helpers::bind_attribute_buffer(vbo, attribute_info)
                    },
                    None => {}
                }
            }

            // for (name, _) in config.attributes.iter() {
            //     let vbo = buffers.get(name).unwrap();
            //     unsafe { gl_helpers::bind_attribute_buffer(*vbo, 0, 3) };
            // }
        })
    }
}

pub fn init() -> Box<Fn() -> Rugl> {
    let window = glutin::Window::new().unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    unsafe { window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    Box::new(Rugl::new)
}

#[cfg(test)]
mod rugl_tests {
    use super::*;

    #[test]
    fn test_the_buider_pattern() {

        let rugl = init();
        let draw = rugl()
            .vert("
                #version 150
                in vec2 position;
                in vec4 color;
                void main() {
                    gl_Position = vec4(position, color[1], 1.0);
                }
            ")
            .frag("
                #version 150
                out vec4 out_color;
                void main() {
                    out_color = vec4(1.0, 1.0, 1.0, 1.0);
                }
            ")
            .attribute("position", vec![
                 0.0,  0.5,
                 0.5, -0.5,
                -0.5, -0.5
            ])
            .finalize();

        draw();
    }
}
