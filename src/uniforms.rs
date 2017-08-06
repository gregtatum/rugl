use super::gl;
use super::gl::types::*;
use super::gl_helpers;

pub trait UniformValue {
    fn set_uniform(&self, GLint, GLenum, i32);
}

impl UniformValue for f32 {
    fn set_uniform(&self, location: GLint, data_type: GLenum, data_size: i32) {
        debug_assert_eq!(data_type, gl::FLOAT);
        debug_assert_eq!(data_size, 1);
        log_draw!("gl::Uniform1f(location:{:?}, {:?})", location, self);
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}

impl UniformValue for [f32; 2] {
    fn set_uniform(&self, location: GLint, data_type: GLenum, data_size: i32) {
        debug_assert_eq!(data_type, gl::FLOAT_VEC2);
        debug_assert_eq!(data_size, 1);
        log_draw!("gl::Uniform2f(location:{:?}, {:?}, {:?})", location, self[0], self[1]);
        unsafe {
            gl::Uniform2f(
                location,
                self[0],
                self[1]
            );
        }
    }
}

impl UniformValue for [f32; 3] {
    fn set_uniform(&self, location: GLint, data_type: GLenum, data_size: i32) {
        debug_assert_eq!(data_type, gl::FLOAT_VEC3);
        debug_assert_eq!(data_size, 1);
        log_draw!("gl::Uniform3f(location:{:?}, {:?}, {:?}, {:?})", location, self[0], self[1], self[2]);
        unsafe {
            gl::Uniform3f(
                location,
                self[0],
                self[1],
                self[2]
            );
        }
    }
}

impl UniformValue for [f32; 4] {
    fn set_uniform(&self, location: GLint, data_type: GLenum, data_size: i32) {
        debug_assert_eq!(data_size, 1);

        match data_type {
            gl::FLOAT_VEC4 => {
                log_draw!("gl::Uniform4f(location:{:?}, {:?}, {:?}, {:?}, {:?})", location, self[0], self[1], self[2], self[3]);
                unsafe {
                    gl::Uniform4f(
                        location,
                        self[0],
                        self[1],
                        self[2],
                        self[3]
                    );
                }
            },
            gl::FLOAT_MAT2 => {
                log_draw!("gl::UniformMatrix2fv(location:{:?}, count:1, transpose:false, *GLfloat) -> {:?}", location, self);
                unsafe {
                    gl::UniformMatrix2fv(
                        location,
                        1 as GLsizei,
                        false as GLboolean, // transpose
                        self.as_ptr() as *const GLfloat
                    );
                }

            },
            _ => panic!("Data types don't match when setting a uniform")
        };
    }
}

impl UniformValue for [f32; 6] {
    fn set_uniform(&self, location: GLint, data_type: GLenum, data_size: i32) {
        debug_assert_eq!(data_size, 1);

        let gl_uniform_setter = match data_type {
            gl::FLOAT_MAT2x3 => {
                log_draw!("gl::UniformMatrix2x3fv(location:{:?}, count:1, transpose:false, *GLfloat) -> {:?}", location, self);
                gl::UniformMatrix2x3fv
            },
            gl::FLOAT_MAT3x2 => {
                log_draw!("gl::UniformMatrix3x2fv(location:{:?}, count:1, transpose:false, *GLfloat) -> {:?}", location, self);
                gl::UniformMatrix3x2fv
            },
            _ => panic!("Data types don't match when setting a uniform")
        };

        unsafe {
            gl_uniform_setter(
                location,
                1 as GLsizei, // count
                false as GLboolean, // transpose
                self as *const GLfloat
            );
        }
    }
}

impl UniformValue for [f32; 8] {
    fn set_uniform(&self, location: GLint, data_type: GLenum, data_size: i32) {
        debug_assert_eq!(data_size, 1);

        let gl_uniform_setter = match data_type {
            gl::FLOAT_MAT2x4 => {
                log_draw!("gl::UniformMatrix2x4fv(location:{:?}, count:1, transpose:false, *GLfloat) -> {:?}", location, self);
                gl::UniformMatrix2x4fv
            },
            gl::FLOAT_MAT4x2 => {
                log_draw!("gl::UniformMatrix4x2fv(location:{:?}, count:1, transpose:false, *GLfloat) -> {:?}", location, self);
                gl::UniformMatrix4x2fv
            },
            _ => panic!("Data types don't match when setting a uniform")
        };

        unsafe {
            gl_uniform_setter(
                location,
                1 as GLsizei, // count
                false as GLboolean, // transpose
                self as *const GLfloat
            );
        }

    }
}

impl UniformValue for [f32; 9] {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::UniformMatrix3fv(location:{:?}, *GLfloat) -> {:?}", location, self);
            gl::UniformMatrix3fv(
                location,
                1 as GLsizei, // count
                false as GLboolean, // transpose
                self as *const GLfloat
            );
        }
    }
}

impl UniformValue for [f32; 12] {
    fn set_uniform(&self, location: GLint, data_type: GLenum, data_size: i32) {
        debug_assert_eq!(data_size, 1);

        let gl_uniform_setter = match data_type {
            gl::FLOAT_MAT3x4 => {
                log_draw!("gl::UniformMatrix3x4fv(location:{:?}, count:1, transpose:false, *GLfloat) -> {:?}", location, self);
                gl::UniformMatrix3x4fv
            },
            gl::FLOAT_MAT4x3 => {
                log_draw!("gl::UniformMatrix4x3fv(location:{:?}, count:1, transpose:false, *GLfloat) -> {:?}", location, self);
                gl::UniformMatrix4x3fv
            },
            _ => panic!("Data types don't match when setting a uniform")
        };

        unsafe {
            gl_uniform_setter(
                location,
                1 as GLsizei, // count
                false as GLboolean, // transpose
                self as *const GLfloat
            );
        }

    }
}

impl UniformValue for [f32; 16] {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::UniformMatrix4fv(location:{:?}, count:1, transpose:false, *GLfloat) -> {:?}", location, self);
            gl::UniformMatrix4fv(
                location,
                1 as GLsizei, // count
                false as GLboolean, // transpose
                self as *const GLfloat
            );
        }
    }
}

impl UniformValue for Vec<f32> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform1fv(location:{:?}, {:?}, *GLfloat) -> {:?}", location, self.len(), self);
            gl::Uniform1fv(
                location,
                self.len() as GLsizei,
                self.as_ptr() as *const GLfloat
            );
        }
    }
}

impl UniformValue for Vec<[f32; 2]> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform2fv(location:{:?}, {:?}, *GLfloat) -> {:?}", location, self.len(), self);
            gl::Uniform2fv(
                location,
                self.len() as GLsizei,
                self.as_ptr() as *const GLfloat
            );
        }
    }
}

impl UniformValue for Vec<[f32; 3]> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform3fv(location:{:?}, count:{:?}, *GLfloat) -> {:?}", location, self.len(), self);
            gl::Uniform3fv(
                location,
                self.len() as GLsizei,
                self.as_ptr() as *const GLfloat
            );
        }
    }
}

impl UniformValue for Vec<[f32; 4]> {
    fn set_uniform(&self, location: GLint, data_type: GLenum, _: i32) {
        match data_type {
            gl::FLOAT_VEC4 => {
                log_draw!("gl::Uniform4fv(location:{:?}, {:?}, *GLfloat) -> {:?}", location, self.len(), self);
                unsafe {
                    gl::Uniform4fv(
                        location,
                        self.len() as GLsizei,
                        self.as_ptr() as *const GLfloat
                    );
                }
            },
            gl::FLOAT_MAT2 => {
                log_draw!("gl::UniformMatrix2fv(location:{:?}, count:{:?}, transpose:false, *GLfloat) -> {:?}", location, self.len(), self);
                unsafe {
                    gl::UniformMatrix2fv(
                        location,
                        self.len() as GLsizei,
                        false as GLboolean, // transpose
                        self.as_ptr() as *const GLfloat
                    );
                }

            },
            _ => panic!("Data types don't match when setting a uniform")
        };
    }
}

impl UniformValue for Vec<[f32; 6]> {
    fn set_uniform(&self, location: GLint, data_type: GLenum, _: i32) {
        let gl_uniform_setter = match data_type {
            gl::FLOAT_MAT2x3 => {
                log_draw!("gl::UniformMatrix2x3fv(location:{:?}, count:{:?}, transpose:false, *GLfloat) -> {:?}", location, self.len(), self);
                gl::UniformMatrix2x3fv
            },
            gl::FLOAT_MAT3x2 => {
                log_draw!("gl::UniformMatrix3x2fv(location:{:?}, count:{:?}, transpose:false, *GLfloat) -> {:?}", location, self.len(), self);
                gl::UniformMatrix3x2fv
            },
            _ => panic!("Data types don't match when setting a uniform")
        };

        log_draw!("gl::UniformMatrix3fv(location:{:?}, count:{:?}, transpose:false, *GLfloat)", location, self);

        unsafe {
            gl_uniform_setter(
                location,
                self.len() as GLsizei,
                false as GLboolean, // transpose
                self.as_ptr() as *const GLfloat
            );
        }
    }
}

impl UniformValue for Vec<[f32; 8]> {
    fn set_uniform(&self, location: GLint, data_type: GLenum, _: i32) {
        let gl_uniform_setter = match data_type {
            gl::FLOAT_MAT2x4 => {
                log_draw!("gl::UniformMatrix2x4fv(location:{:?}, count:{:?}, transpose:false, *GLfloat) -> {:?}", location, self.len(), self);
                gl::UniformMatrix2x4fv
            },
            gl::FLOAT_MAT4x2 => {
                log_draw!("gl::UniformMatrix4x2fv(location:{:?}, count:{:?}, transpose:false, *GLfloat) -> {:?}", location, self.len(), self);
                gl::UniformMatrix4x2fv
            },
            _ => panic!("Data types don't match when setting a uniform")
        };

        unsafe {
            gl_uniform_setter(
                location,
                self.len() as GLsizei,
                false as GLboolean, // transpose
                self.as_ptr() as *const GLfloat
            );
        }
    }
}

impl UniformValue for Vec<[f32; 9]> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::UniformMatrix3fv(location:{:?}, count:{:?}, transpose:false, *GLfloat) -> {:?}", location, self.len(), self);
            gl::UniformMatrix3fv(
                location,
                self.len() as GLsizei,
                false as GLboolean, // transpose
                self.as_ptr() as *const GLfloat
            );
        }
    }
}

impl UniformValue for Vec<[f32; 12]> {
    fn set_uniform(&self, location: GLint, data_type: GLenum, _: i32) {
        let gl_uniform_setter = match data_type {
            gl::FLOAT_MAT3x4 => {
                log_draw!("gl::UniformMatrix3x4fv(location:{:?}, count:{:?}, transpose:false, *GLfloat) -> {:?}", location, self.len(), self);
                gl::UniformMatrix3x4fv
            },
            gl::FLOAT_MAT4x3 => {
                log_draw!("gl::UniformMatrix4x3fv(location:{:?}, count:{:?}, transpose:false, *GLfloat) -> {:?}", location, self.len(), self);
                gl::UniformMatrix4x3fv
            },
            _ => panic!("Data types don't match when setting a uniform")
        };

        unsafe {
            gl_uniform_setter(
                location,
                self.len() as GLsizei,
                false as GLboolean, // transpose
                self.as_ptr() as *const GLfloat
            );
        }
    }
}

impl UniformValue for Vec<[f32; 16]> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::UniformMatrix4fv(location:{:?}, count:{:?}, transpose:false, *GLfloat) -> {:?}", location, self.len(), self);
            gl::UniformMatrix4fv(
                location,
                self.len() as GLsizei,
                false as GLboolean, // transpose
                self.as_ptr() as *const GLfloat
            );
        }
    }
}

impl UniformValue for i32 {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform1i(location:{:?}, {:?})", location, self);
            gl::Uniform1i(location, *self);
        }
    }
}

impl UniformValue for [i32; 2] {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform2i(location:{:?}, {:?}, {:?})", location, self[0], self[1]);
            gl::Uniform2i(
                location,
                self[0],
                self[1]
            );
        }
    }
}

impl UniformValue for [i32; 3] {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform3i(location:{:?}, {:?}, {:?}, {:?})", location, self[0], self[1], self[2]);
            gl::Uniform3i(
                location,
                self[0],
                self[1],
                self[2]
            );
        }
    }
}

impl UniformValue for [i32; 4] {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform3i(location:{:?}, {:?}, {:?}, {:?}, {:?})", location, self[0], self[1], self[2], self[3]);
            gl::Uniform4i(
                location,
                self[0],
                self[1],
                self[2],
                self[3]
            );
        }
    }
}

impl UniformValue for u32 {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform1ui(location:{:?}, {:?})", location, self);
            gl::Uniform1ui(location, *self);
        }
    }
}

impl UniformValue for [u32; 2] {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform2ui(location:{:?}, {:?}, {:?})", location, self[0], self[1]);
            gl::Uniform2ui(
                location,
                self[0],
                self[1]
            );
        }
    }
}

impl UniformValue for [u32; 3] {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform3ui(location:{:?}, {:?}, {:?}, {:?})", location, self[0], self[1], self[2]);
            gl::Uniform3ui(
                location,
                self[0],
                self[1],
                self[2]
            );
        }
    }
}

impl UniformValue for [u32; 4] {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform4ui(location:{:?}, {:?}, {:?}, {:?}, {:?})", location, self[0], self[1], self[2], self[3]);
            gl::Uniform4ui(
                location,
                self[0],
                self[1],
                self[2],
                self[3]
            );
        }
    }
}

impl UniformValue for Vec<i32> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform1iv(location:{:?}, {:?}, *GLint) -> {:?}", location, self.len(), self);
            gl::Uniform1iv(
                location,
                self.len() as GLsizei,
                self.as_ptr() as *const GLint
            );
        }
    }
}

impl UniformValue for Vec<[i32; 2]> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform2iv(location:{:?}, {:?}, *GLint) -> {:?}", location, self.len(), self);
            gl::Uniform2iv(
                location,
                self.len() as GLsizei,
                self.as_ptr() as *const GLint
            );
        }
    }
}

impl UniformValue for Vec<[i32; 3]> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform3iv(location:{:?}, {:?}, *GLint) -> {:?}", location, self.len(), self);
            gl::Uniform3iv(
                location,
                self.len() as GLsizei,
                self.as_ptr() as *const GLint
            );
        }
    }
}

impl UniformValue for Vec<[i32; 4]> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform4iv(location:{:?}, {:?}, *GLint) -> {:?}", location, self.len(), self);
            gl::Uniform4iv(
                location,
                self.len() as GLsizei,
                self.as_ptr() as *const GLint
            );
        }
    }
}

impl UniformValue for Vec<u32> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform1uiv(location:{:?}, {:?}, *GLint) -> {:?}", location, self.len(), self);
            gl::Uniform1uiv(
                location,
                self.len() as GLsizei,
                self.as_ptr() as *const GLuint
            );
        }
    }
}

impl UniformValue for Vec<[u32; 2]> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform2uiv(location:{:?}, {:?}, *GLint) -> {:?}", location, self.len(), self);
            gl::Uniform2uiv(
                location,
                self.len() as GLsizei,
                self.as_ptr() as *const GLuint
            );
        }
    }
}

impl UniformValue for Vec<[u32; 3]> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform3uiv(location:{:?}, {:?}, *GLint) -> {:?}", location, self.len(), self);
            gl::Uniform3uiv(
                location,
                self.len() as GLsizei,
                self.as_ptr() as *const GLuint
            );
        }
    }
}

impl UniformValue for Vec<[u32; 4]> {
    fn set_uniform(&self, location: GLint, _: GLenum, _: i32) {
        unsafe {
            log_draw!("gl::Uniform4uiv(location:{:?}, {:?}, *GLint) -> {:?}", location, self.len(), self);
            gl::Uniform4uiv(
                location,
                self.len() as GLsizei,
                self.as_ptr() as *const GLuint
            );
        }
    }
}
