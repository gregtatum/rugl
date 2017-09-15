use super::gl_helpers;
use super::buffers::{BufferableData, BufferableElementsData};
use super::gl::types::*;
use super::gl;
use super::rugl;
use super::uniforms::UniformValue;
use super::Primitive;

use std::collections::HashMap;
use std::string;
use std::ptr;

pub struct Draw {
    pub vert: Option<&'static str>,
    pub frag: Option<&'static str>,
    pub attributes: Vec<(String, GLuint)>,
    pub elements: Option<GLuint>,
    pub uniform_setters: HashMap<
        String,
        Box<Fn(&rugl::Environment) -> Box<UniformValue>>
    >,
    pub primitive: Primitive,
    pub count: i32
}

impl Draw {
    pub fn new() -> Draw {
        Draw {
            vert: None,
            frag: None,
            attributes: Vec::new(),
            elements: None,
            uniform_setters: HashMap::new(),
            primitive: Primitive::Triangles,
            count: 0
        }
    }

    pub fn add_uniform(
        &mut self,
        name: &str,
        setter: Box<Fn(&rugl::Environment) -> Box<UniformValue>>
    ) {
        self.uniform_setters.insert(name.to_string(), setter);
    }

    pub fn add_attribute(
        &mut self,
        name: &str,
        vertices: &BufferableData
    ) {
        self.attributes.push(
            (name.to_string(), vertices.to_buffer())
        );
    }

    pub fn set_primitive(&mut self, primitive: Primitive) {
        match self.elements {
            Some(_) => panic!("Primitives must be set before elements on draw commands in order to properly create a buffer from a borrowed value."),
            None => { self.primitive = primitive }
        };
    }

    pub fn set_elements(
        &mut self, elements: &BufferableElementsData
    ) {
        self.elements = Some(elements.to_buffer(&self.primitive));
        self.count = elements.get_count(&self.primitive);
    }

    pub fn finalize(mut self) -> Box<Fn(&rugl::Environment)> {
        let vertex_shader = match self.vert {
            Some(vert) => Some(gl_helpers::compile_shader(vert, gl::VERTEX_SHADER)),
            None => None
        };
        let fragment_shader = match self.frag {
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
            let attribute_config = self.attributes
                .iter()
                .find(|attribute_tuple| {
                    attribute_tuple.0 == attribute_info.name
                });

            match attribute_config {
                Some(&(_, ref buffer)) => { Some(*buffer) },
                _ => None
            }
        }).collect();

        let count = self.count;

        let do_draw_elements = match self.elements {
            Some(_) => true,
            None => false
        };

        let draw_mode = self.primitive.to_gl_enum();

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

                match self.elements {
                    Some(elements) => {
                        log_draw!("gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, {:?})", elements);
                        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, elements) }
                    },
                    None => {}
                }

                // Un-bind the vao, now when we bind it again, it will restore the state
                // of our shader.
                gl_helpers::bind_vao(0);
                Some(vao)
            },
            None => None
        };

        // Match up the uniform setters with their appropriate uniform infos from the program.
        let matched_uniform_setters = match program {
            Some(program) => {
                let mut results = Vec::new();
                let uniform_infos = gl_helpers::get_uniforms(program);
                for uniform_info in uniform_infos.iter() {
                    match self.uniform_setters.remove(&uniform_info.name) {
                        Some(setter) => {
                            let location = uniform_info.location as i32;
                            let data_type = uniform_info.data_type;
                            let data_size = uniform_info.data_size;
                            results.push(
                                Box::new(move |environment: &rugl::Environment| {
                                    (
                                        *setter(&environment)
                                    )
                                    .set_uniform(
                                        location,
                                        data_type,
                                        data_size
                                    );
                                    check_gl_errors!();
                                })
                            );
                        }
                        None => {}
                    };
                }
                results
            },
            None => Vec::new()
        };

        return Box::new(move |environment: &rugl::Environment| {
            #[cfg(feature = "debug_draw")]
            println!("----------------------------------------------------");
            match program {
                Some(program) => {
                    gl_helpers::use_program(program);
                    gl_helpers::bind_vao(vao.unwrap());
                    for setter in matched_uniform_setters.iter() {
                        setter(environment);
                    }

                    match do_draw_elements {
                        true => gl_helpers::draw_elements(draw_mode, count),
                        false => gl_helpers::draw_arrays(draw_mode, 0, count)
                    };
                },
                None => {}
            };
        })
    }
}

#[cfg(test)]
#[macro_use]
mod tests {
    use super::*;

    static VERT: &str = "
        #version 150
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    ";
    static FRAG: &str = "
        #version 150
        out vec4 out_color;
        void main() {
            out_color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    ";

    #[test]
    fn empty_macro() {
        let rugl = rugl::init_headless();
        #[allow(unused_variables)]
        let draw = rugl!(rugl.draw, {});
    }

    #[test]
    #[should_panic]
    fn macro_with_shader() {
        let rugl = rugl::init_headless();
        #[allow(unused_variables)]
        let draw = rugl!(rugl.draw, {
            vert => VERT,
            frag => FRAG
        });
    }

    #[test]
    fn macro_with_uniforms() {
        let rugl = rugl::init_headless();
        #[allow(unused_variables)]
        let draw = rugl!(rugl.draw, {
            uniforms => {
                color => |_| Box::new([1.0, 0.0, 0.0]),
                color2 => |_| Box::new([1.0, 0.0, 0.0])
            }
        });
    }

    #[test]
    #[should_panic]
    fn macro_with_attributes() {
        let rugl = rugl::init_headless();
        #[allow(unused_variables)]
        let draw = rugl!(rugl.draw, {
            attributes => {
                id => {
                    &((0..10).map(|i| {
                        (i as f32 / 3.0).floor()
                    }).collect::<Vec<f32>>())
                }
            }
        });
    }

    #[test]
    #[should_panic]
    fn macro_with_elements() {
        let rugl = rugl::init_headless();
        #[allow(unused_variables)]
        let draw = rugl!(rugl.draw, {
            elements => { &vec![0u32, 1, 2] }
        });
    }

    #[test]
    fn macro_with_count() {
        let rugl = rugl::init_headless();
        #[allow(unused_variables)]
        let draw = rugl!(rugl.draw, {
            count => 5
        });
    }
}
