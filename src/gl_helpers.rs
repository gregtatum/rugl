use super::gl;
use super::gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::fmt;

pub struct AttributeInfo {
    pub name: String,
    pub index: GLuint,
    pub data_size: GLint,
    pub data_type: GLenum,
}

impl fmt::Debug for AttributeInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "AttributeInfo {{ name: \"{}\": data_size: {}, data_type: {} }}",
            self.name, self.data_size, self.data_type
        )
    }
}

pub unsafe fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    let shader = gl::CreateShader(shader_type);

    // Attempt to compile the shader
    let c_str = CString::new(source.as_bytes()).unwrap();
    gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    // Get the compile status
    let mut status = gl::FALSE as GLint;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

    // Fail on error
    if status != (gl::TRUE as GLint) {
        let mut len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = Vec::with_capacity(len as usize);
        buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
        gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
        panic!("There was an error compiling the shader: {}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
    }
    shader
}

pub unsafe fn link_program(vertex_shader: &GLuint, fragment_shader: &GLuint) -> GLuint {
    let program = gl::CreateProgram();
    gl::AttachShader(program, *vertex_shader);
    gl::AttachShader(program, *fragment_shader);
    gl::LinkProgram(program);
    // Get the link status
    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    // Fail on error
    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = Vec::with_capacity(len as usize);
        buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
        gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
        panic!("There was an error linking the shader: {}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
    }
    program
}

pub unsafe fn create_buffer(vertex_data: &[GLfloat]) -> GLuint {
    let mut vbo = mem::uninitialized();
    // Create a Vertex Buffer Object and copy the vertex data to it.
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        mem::transmute(&vertex_data[0]),
        gl::STATIC_DRAW
    );

    // Make sure the gl state is clean.
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    vbo
}

pub unsafe fn bind_attribute_buffer(
    vbo: GLuint,
    attribute_info: &AttributeInfo
) {
    // Enable the slot in the shader for this attribute.
    // TODO - This should be abstracted up above this function.
    gl::EnableVertexAttribArray(attribute_info.index);

    // Bind the buffer of data that's going in that slot.
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

    // Define how the pointers look up the information in the buffer.
    gl::VertexAttribPointer(
        attribute_info.index,
        attribute_info.data_size,
        attribute_info.data_type,
        gl::FALSE, // normalize
        0, // stride
        ptr::null() // offset
        // (2 * mem::size_of::<f32>()) as *const () as *const _)
    );
}

// let attribute_name = CString::new("position").unwrap().as_ptr();

pub unsafe fn set_vbo(program: &GLuint, attribute_name: &CString) {
    // Use shader program
    gl::UseProgram(*program);
    gl::BindFragDataLocation(
        *program, // program
        0, // color
        CString::new("out_color").unwrap().as_ptr() // name
    );

    // Specify the layout of the vertex data
    let position_attribute = gl::GetAttribLocation(*program, attribute_name.as_ptr());
    gl::EnableVertexAttribArray(position_attribute as GLuint);
    gl::VertexAttribPointer(
        position_attribute as GLuint, // index
        2, // size
        gl::FLOAT, // type
        gl::FALSE as GLboolean, // normalized
        0, // stride
        ptr::null() // pointer
    );
}

pub unsafe fn get_attribute_count(program: GLuint) -> GLint {
    // Get the count of attributes in our shader.
    let mut attribute_count: GLint = mem::uninitialized();
    gl::GetProgramiv(program, gl::ACTIVE_ATTRIBUTES, &mut attribute_count);
    attribute_count
}

pub unsafe fn get_attribute_info(
    program: GLuint,
    attribute_index: GLint
) -> AttributeInfo {
    let max_name_length: GLsizei = 127;
    let mut name_buffer: Vec<u8> = Vec::with_capacity(128);

    // Write the attribute information into these values.
    let mut name_length: GLsizei = mem::uninitialized();
    let mut data_size: GLint = mem::uninitialized();
    let mut data_type: GLenum = mem::uninitialized();

    gl::GetActiveAttrib(
        program,
        attribute_index as GLuint,
        max_name_length,
        &mut name_length,
        &mut data_size,
        &mut data_type,
        name_buffer.as_mut_ptr() as *mut gl::types::GLchar
    );

    name_buffer.set_len(name_length as usize);
    let name = String::from_utf8(name_buffer).unwrap();

    AttributeInfo {
        name: name,
        index: attribute_index as GLuint,
        data_size: data_size,
        data_type: data_type
    }
}

pub fn get_program_attributes(program: GLuint) -> Vec<AttributeInfo> {
    let mut attributes: Vec<AttributeInfo> = Vec::new();
    let attribute_count = unsafe { get_attribute_count(program) };
    for attribute_index in 0..attribute_count {
        let attribute_info = unsafe { get_attribute_info(program, attribute_index) };
        if attribute_info.name.starts_with("gl_") {
            continue;
        }
        attributes.push(attribute_info);
    }
    attributes
}
