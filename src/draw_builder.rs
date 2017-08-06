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

pub struct DrawConfig<> {
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

pub struct DrawBuilder {
    pub config: DrawConfig
}

impl DrawBuilder {
    pub fn new() -> DrawBuilder {
        DrawBuilder {
            config: DrawConfig {
                vert: None,
                frag: None,
                attributes: Vec::new(),
                elements: None,
                uniform_setters: HashMap::new(),
                primitive: Primitive::Triangles,
                count: 0
            }
        }
    }

    pub fn vert(mut self, source: &'static str) -> DrawBuilder {
        self.config.vert = Some(source);
        self
    }

    pub fn frag(mut self, source: &'static str) -> DrawBuilder {
        self.config.frag = Some(source);
        self
    }

    pub fn uniform(mut self, name: &str, setter: Box<Fn(&rugl::Environment) -> Box<UniformValue>>) -> DrawBuilder {
        self.config.uniform_setters.insert(name.to_string(), setter);
        self
    }

    pub fn attribute(
        mut self, name: &str, vertices: &BufferableData
    ) -> DrawBuilder {
        self.config.attributes.push(
            (name.to_string(), vertices.to_buffer())
        );
        self
    }

    pub fn primitive(mut self, primitive: Primitive) -> DrawBuilder {
        match self.config.elements {
            Some(_) => panic!(".primitive() must be called before .elements() in order to properly create a buffer from a borrowed value."),
            None => { self.config.primitive = primitive }
        };
        self
    }

    pub fn elements(
        mut self, elements: &BufferableElementsData
    ) -> DrawBuilder {
        self.config.elements = Some(elements.to_buffer(&self.config.primitive));
        self.config.count = elements.get_count(&self.config.primitive);
        self
    }

    pub fn count(mut self, count: i32) -> DrawBuilder {
        self.config.count = count;
        self
    }

    pub fn finalize(self) -> Box<Fn(&rugl::Environment)> {
        let mut config = self.config;
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

        let do_draw_elements = match config.elements {
            Some(_) => true,
            None => false
        };

        let draw_mode = config.primitive.to_gl_enum();

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

                match config.elements {
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
                    match config.uniform_setters.remove(&uniform_info.name) {
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
