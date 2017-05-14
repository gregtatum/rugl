use super::gl_helpers;
use super::buffers::BufferableData;
use super::gl::types::*;
use super::gl;
use super::rugl;
use super::uniforms::UniformValue;
use std::collections::HashMap;
use std::string;

pub struct DrawConfig<> {
    pub vert: Option<&'static str>,
    pub frag: Option<&'static str>,
    pub attributes: Vec<(String, GLuint)>,
    pub uniform_setters: HashMap<
        String,
        Box<Fn(&rugl::Environment) -> Box<UniformValue>>
    >,
    // pub uniforms: Vec<(String, GL
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
                uniform_setters: HashMap::new(),
                count: 0
            }
        }
    }

    pub fn vert(&mut self, source: &'static str) -> &mut DrawBuilder {
        self.config.vert = Some(source);
        self
    }

    pub fn frag(&mut self, source: &'static str) -> &mut DrawBuilder {
        self.config.frag = Some(source);
        self
    }

    pub fn uniform(&mut self, name: &str, setter: Box<Fn(&rugl::Environment) -> Box<UniformValue>>) -> &mut DrawBuilder {
        self.config.uniform_setters.insert(name.to_string(), setter);
        self
    }

    pub fn attribute(
        &mut self, name: &str, vertices: &BufferableData
    ) -> &mut DrawBuilder {
        // gl_helpers::log_draw!("Adding attribute {:?}", name);
        self.config.attributes.push(
            (name.to_string(), vertices.to_buffer())
        );
        self
    }

    pub fn count(&mut self, count: i32) -> &mut DrawBuilder {
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

        // Match up the uniform setters with their appropriate uniform infos from the program.
        let matched_uniform_setters = match program {
            Some(program) => {
                let mut results = Vec::new();
                let uniform_infos = gl_helpers::get_uniforms(program);
                for uniform_info in uniform_infos.iter() {
                    match config.uniform_setters.remove(&uniform_info.name) {
                        Some(setter) => {
                            let index = uniform_info.index as i32;
                            let data_type = uniform_info.data_type;
                            let data_size = uniform_info.data_size;
                            results.push(
                                Box::new(move |environment: &rugl::Environment| {
                                    (*setter(&environment)).set_uniform(
                                        index,
                                        data_type,
                                        data_size
                                    );
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
                    for setter in matched_uniform_setters.iter() {
                        setter(environment);
                    }
                    gl_helpers::bind_vao(vao.unwrap());
                    gl_helpers::draw_arrays(gl::TRIANGLES, 0, count);
                },
                None => {}
            };
        })
    }
}
