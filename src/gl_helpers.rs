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
    // The strict type enum, e.g. gl::FLOAT_VEC2
    pub type_enum: GLenum,
    // The type of the attribute, e.g. gl::FLOAT
    pub data_type: GLenum,
    // How many of the types this has, e.g.
    pub data_size: GLint,
}

impl fmt::Debug for AttributeInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "AttributeInfo {{\
            \n  name: \"{}\",\
            \n  type_enum: {},\
            \n  data_type: {},\
            \n  data_size: {}\
            \n}}",
            self.name,
            gl_attribute_enum_to_string(self.type_enum),
            gl_attribute_enum_to_string(self.data_type),
            self.data_size
        )
    }
}

pub struct UniformInfo {
    pub name: String,
    pub index: GLuint,
    // The type enum, e.g. gl::FLOAT_VEC2
    pub data_type: GLenum,
    // The size of an array of values, typically 1.
    pub data_size: i32,
}

impl fmt::Debug for UniformInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "UniformInfo {{\
            \n  name: \"{}\",\
            \n  data_type: {},\
            \n  data_size: {}\
            \n}}",
            self.name,
            gl_attribute_enum_to_string(self.data_type),
            self.data_size
        )
    }
}

pub fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        log_draw!("gl::CreateShader(shader_type:{})", gl_shader_type_enum_to_string(shader_type));
        let shader = gl::CreateShader(shader_type);
        log_draw!("gl::CreateShader -> {:?}", shader);

        // Attempt to compile the shader
        let c_str = CString::new(source.as_bytes()).unwrap();
        log_draw!("\"{}\n\"", source);
        log_draw!("gl::ShaderSource(shader:{:?}, count:{:?}, source:{:?}, ptr::null())", shader, 1, &c_str.as_ptr());
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());

        log_draw!("gl::CompileShader({:?})", shader);
        gl::CompileShader(shader);

        // Get the compile status
        let mut status: GLint = mem::uninitialized();
        log_draw!("gl::GetShaderiv(shader:{:?}, gl::COMPILE_STATUS, *status)", shader);
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        log_draw!("    status -> {}", status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut log_length: GLint = mem::uninitialized();
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
            let mut log = Vec::with_capacity(log_length as usize);
            log.set_len((log_length as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, log_length, ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);
            panic!("There was an error compiling the shader: {}", str::from_utf8(&log).ok().expect("ShaderInfoLog not valid utf8"));
        }
        shader
    }
}

pub fn link_program(vertex_shader: &GLuint, fragment_shader: &GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        log_draw!("gl::CreateProgram() -> {:?}", program);
        log_draw!("gl::AttachShader(program:{:?}, vertex_shader:{:?})", program, *vertex_shader);
        gl::AttachShader(program, *vertex_shader);
        log_draw!("gl::AttachShader(program:{:?}, fragment_shader:{:?})", program, *fragment_shader);
        gl::AttachShader(program, *fragment_shader);
        log_draw!("gl::LinkProgram(program:{:?})", program);
        gl::LinkProgram(program);

        // Get the link status
        let mut status: GLint = mem::uninitialized();
        log_draw!("gl::GetProgramiv(program:{:?}, gl::LINK_STATUS, &mut status)", program);
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        log_draw!("    status -> {}", status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut log_length: GLint = mem::uninitialized();
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);
            let mut log_buffer = Vec::with_capacity(log_length as usize);
            log_buffer.set_len((log_length as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, log_length, ptr::null_mut(), log_buffer.as_mut_ptr() as *mut GLchar);
            panic!("There was an error linking the shader: {}", str::from_utf8(&log_buffer).ok().expect("ProgramInfoLog not valid utf8"));
        }
        program
    }
}

pub fn use_program(program: GLuint) {
    log_draw!("gl::UseProgram(program:{:?})", program);
    unsafe {
        gl::UseProgram(program);
    };
}

pub fn create_buffer(vertex_data: &[GLfloat]) -> GLuint {
    unsafe {
        // Create a vertex buffer object and copy the vertex data to it.
        let mut buffer: GLuint = mem::uninitialized();
        log_draw!("gl::GenBuffers(size:1, *buffer)");
        gl::GenBuffers(1, &mut buffer);
        log_draw!("    buffer -> {}", buffer);

        log_draw!("gl::BindBuffer(gl::ARRAY_BUFFER, buffer:{:?})", buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);

        let size = (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
        log_draw!(
            "gl::BufferData(gl::ARRAY_BUFFER, size:{:?}, *data, gl::STATIC_DRAW)",
            size
        );
        // log_draw!("    vertex_data: {:?}", vertex_data);
        log_draw!("    GLFloat size: {}", mem::size_of::<GLfloat>());
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size,
            mem::transmute(&vertex_data[0]),
            gl::STATIC_DRAW
        );

        // Make sure the gl state is clean.
        log_draw!("gl::BindBuffer(gl::ARRAY_BUFFER, buffer:0)");
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        buffer
    }
}

// TODO - De-duplicate this code so that it works with both GLfloat and GLuint.
pub fn create_buffer_u32(vertex_data: &[GLuint]) -> GLuint {
    unsafe {
        // Create a vertex buffer object and copy the vertex data to it.
        let mut buffer: GLuint = mem::uninitialized();
        log_draw!("gl::GenBuffers(size:1, *buffer)");
        gl::GenBuffers(1, &mut buffer);
        log_draw!("    buffer -> {}", buffer);

        log_draw!("gl::BindBuffer(gl::ARRAY_BUFFER, buffer:{:?})", buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);

        let size = (vertex_data.len() * mem::size_of::<GLuint>()) as GLsizeiptr;
        log_draw!(
            "gl::BufferData(gl::ARRAY_BUFFER, size:{:?}, *data, gl::STATIC_DRAW)",
            size
        );
        // log_draw!("    vertex_data: {:?}", vertex_data);
        log_draw!("    GLFloat size: {}", mem::size_of::<GLuint>());
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size,
            mem::transmute(&vertex_data[0]),
            gl::STATIC_DRAW
        );

        // Make sure the gl state is clean.
        log_draw!("gl::BindBuffer(gl::ARRAY_BUFFER, buffer:0)");
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        buffer
    }
}

pub fn create_vao() -> GLuint {
    unsafe {
        // Create Vertex Array Object
        let mut vao: GLuint = mem::uninitialized();
        log_draw!("gl::GenVertexArrays(size:1, *vao)");
        gl::GenVertexArrays(1, &mut vao);
        log_draw!("    vao -> {}", vao);
        vao
    }
}

pub fn bind_vao(vao: GLuint) {
    unsafe {
        log_draw!("gl::BindVertexArray({})", vao);
        gl::BindVertexArray(vao);
    }
}

pub fn bind_attribute_buffer(
    vbo: GLuint,
    attribute_info: &AttributeInfo
) {
    unsafe {
        // Bind the buffer of data that's going in that slot.
        log_draw!("gl::BindBuffer(gl::ARRAY_BUFFER, {})", vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Enable the slot in the shader for this attribute.
        log_draw!("gl::EnableVertexAttribArray({})", attribute_info.index);
        gl::EnableVertexAttribArray(attribute_info.index);

        // Define how the pointers look up the information in the buffer.
        log_draw!(
            "gl::VertexAttribPointer(index:{}, size:{}, type:{}, normalize:gl::FALSE, stride:0,\
            \n                        offset: ptr::null())",
            attribute_info.index,
            attribute_info.data_size,
            gl_attribute_enum_to_string(attribute_info.data_type)
        );
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
}

pub fn get_attribute_count(program: GLuint) -> GLint {
    unsafe {
        // Get the count of attributes in our shader.
        let mut attribute_count: GLint = mem::uninitialized();
        gl::GetProgramiv(program, gl::ACTIVE_ATTRIBUTES, &mut attribute_count);
        attribute_count
    }
}

pub fn get_uniform_count(program: GLuint) -> GLint {
    unsafe {
        // Get the count of uniforms in our shader.
        let mut uniform_count: GLint = mem::uninitialized();
        gl::GetProgramiv(program, gl::ACTIVE_UNIFORMS, &mut uniform_count);
        uniform_count
    }
}

pub fn get_attribute_info(
    program: GLuint,
    attribute_index: GLint
) -> AttributeInfo {
    unsafe {
        let max_name_length: GLsizei = 127;
        let mut name_buffer: Vec<u8> = Vec::with_capacity(128);

        // Write the attribute information into these values.
        let mut name_length: GLsizei = mem::uninitialized();
        let mut data_size: GLint = mem::uninitialized();
        let mut data_type: GLenum = mem::uninitialized();

        log_draw!(
            "gl::GetActiveAttrib(program:{}, attribute:{}, max_name_length:{}, *name_length, \
            \n                    *data_size, *data_type, *name_buffer)",
            program,
            attribute_index as GLuint,
            max_name_length
        );
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
        log_draw!("    name -> {:?}", name);
        log_draw!("    name_length -> {}", name_length);
        log_draw!("    data_size -> {}", data_size);
        log_draw!("    data_type -> {}", gl_attribute_enum_to_string(data_type));

        let info = AttributeInfo {
            name: name,
            index: attribute_index as GLuint,
            data_type: get_attribute_type(data_type),
            data_size: get_attribute_type_size(data_type),
            type_enum: data_type
        };

        log_draw!("{:?}", info);

        info
    }
}

pub fn get_uniform_info(
    program: GLuint,
    uniform_index: GLint
) -> UniformInfo {
    unsafe {
        let max_name_length: GLsizei = 127;
        let mut name_buffer: Vec<u8> = Vec::with_capacity(128);

        // Write the uniform information into these values.
        let mut name_length: GLsizei = mem::uninitialized();
        let mut data_size: GLsizei = mem::uninitialized();
        let mut data_type: GLenum = mem::uninitialized();

        log_draw!(
            "gl::GetActiveUniform(program:{}, uniform:{}, max_name_length:{}, *name_length, \
            \n                    *data_size, *data_type, *name_buffer)",
            program,
            uniform_index as GLuint,
            max_name_length
        );

        gl::GetActiveUniform(
            program,
            uniform_index as GLuint,
            max_name_length,
            &mut name_length,
            &mut data_size,
            &mut data_type,
            name_buffer.as_mut_ptr() as *mut gl::types::GLchar
        );

        name_buffer.set_len(name_length as usize);
        let name = String::from_utf8(name_buffer).unwrap();
        log_draw!("    name -> {:?}", name);
        log_draw!("    name_length -> {}", name_length);
        log_draw!("    data_size -> {}", data_size);
        log_draw!("    data_type -> {}", gl_attribute_enum_to_string(data_type));

        let info = UniformInfo {
            name: name,
            index: uniform_index as GLuint,
            data_type: data_type,
            data_size: data_size
        };

        log_draw!("{:?}", info);

        info
    }
}

pub fn get_program_attributes(program: GLuint) -> Vec<AttributeInfo> {
    let mut attributes: Vec<AttributeInfo> = Vec::new();
    let attribute_count = get_attribute_count(program);
    for attribute_index in 0..attribute_count {
        let attribute_info = get_attribute_info(program, attribute_index);
        if attribute_info.name.starts_with("gl_") {
            continue;
        }
        attributes.push(attribute_info);
    }
    attributes
}

pub fn get_uniforms(program: GLuint) -> Vec<UniformInfo> {
    let mut uniforms: Vec<UniformInfo> = Vec::new();
    let uniform_count = get_uniform_count(program);
    for uniform_index in 0..uniform_count {
        let uniform_info = get_uniform_info(program, uniform_index);
        uniforms.push(uniform_info);
    }
    uniforms
}

pub fn get_attribute_type_size(data_type: GLenum) -> GLint {
    match data_type {
        gl::FLOAT => 1,
        gl::FLOAT_VEC2 => 2,
        gl::FLOAT_VEC3 => 3,
        gl::FLOAT_VEC4 => 4,
        gl::FLOAT_MAT2 => 4,
        gl::FLOAT_MAT3 => 9,
        gl::FLOAT_MAT4 => 16,
        gl::FLOAT_MAT2x3 => 6,
        gl::FLOAT_MAT2x4 => 8,
        gl::FLOAT_MAT3x2 => 6,
        gl::FLOAT_MAT3x4 => 12,
        gl::FLOAT_MAT4x2 => 8,
        gl::FLOAT_MAT4x3 => 12,
        gl::INT => 1,
        gl::INT_VEC2 => 2,
        gl::INT_VEC3 => 3,
        gl::INT_VEC4 => 4,
        gl::UNSIGNED_INT => 1,
        gl::UNSIGNED_INT_VEC2 => 2,
        gl::UNSIGNED_INT_VEC3 => 3,
        gl::UNSIGNED_INT_VEC4 => 4,
        gl::DOUBLE => 1,
        gl::DOUBLE_VEC2 => 2,
        gl::DOUBLE_VEC3 => 3,
        gl::DOUBLE_VEC4 => 4,
        gl::DOUBLE_MAT2 => 2,
        gl::DOUBLE_MAT3 => 3,
        gl::DOUBLE_MAT4 => 4,
        gl::DOUBLE_MAT2x3 => 6,
        gl::DOUBLE_MAT2x4 => 8,
        gl::DOUBLE_MAT3x2 => 6,
        gl::DOUBLE_MAT3x4 => 12,
        gl::DOUBLE_MAT4x2 => 8,
        gl::DOUBLE_MAT4x3 => 12,
        _ => panic!("Unknown gl enum returned")
    }
}

pub fn get_attribute_type(data_type: GLenum) -> GLenum {
    match data_type {
        gl::FLOAT => gl::FLOAT,
        gl::FLOAT_VEC2 => gl::FLOAT,
        gl::FLOAT_VEC3 => gl::FLOAT,
        gl::FLOAT_VEC4 => gl::FLOAT,
        gl::FLOAT_MAT2 => gl::FLOAT,
        gl::FLOAT_MAT3 => gl::FLOAT,
        gl::FLOAT_MAT4 => gl::FLOAT,
        gl::FLOAT_MAT2x3 => gl::FLOAT,
        gl::FLOAT_MAT2x4 => gl::FLOAT,
        gl::FLOAT_MAT3x2 => gl::FLOAT,
        gl::FLOAT_MAT3x4 => gl::FLOAT,
        gl::FLOAT_MAT4x2 => gl::FLOAT,
        gl::FLOAT_MAT4x3 => gl::FLOAT,
        gl::INT => gl::INT,
        gl::INT_VEC2 => gl::INT,
        gl::INT_VEC3 => gl::INT,
        gl::INT_VEC4 => gl::INT,
        gl::UNSIGNED_INT => gl::UNSIGNED_INT,
        gl::UNSIGNED_INT_VEC2 => gl::UNSIGNED_INT,
        gl::UNSIGNED_INT_VEC3 => gl::UNSIGNED_INT,
        gl::UNSIGNED_INT_VEC4 => gl::UNSIGNED_INT,
        gl::DOUBLE => gl::DOUBLE,
        gl::DOUBLE_VEC2 => gl::DOUBLE,
        gl::DOUBLE_VEC3 => gl::DOUBLE,
        gl::DOUBLE_VEC4 => gl::DOUBLE,
        gl::DOUBLE_MAT2 => gl::DOUBLE,
        gl::DOUBLE_MAT3 => gl::DOUBLE,
        gl::DOUBLE_MAT4 => gl::DOUBLE,
        gl::DOUBLE_MAT2x3 => gl::DOUBLE,
        gl::DOUBLE_MAT2x4 => gl::DOUBLE,
        gl::DOUBLE_MAT3x2 => gl::DOUBLE,
        gl::DOUBLE_MAT3x4 => gl::DOUBLE,
        gl::DOUBLE_MAT4x2 => gl::DOUBLE,
        gl::DOUBLE_MAT4x3 => gl::DOUBLE,
        _ => panic!("Unknown gl enum returned")
    }
}

pub fn draw_arrays(mode: GLenum, start: GLint, count: GLsizei) {
    unsafe {
        log_draw!(
            "gl::DrawArrays({}, {}, {:?})",
            gl_draw_mode_enum_to_string(mode),
            start,
            count
        );
        gl::DrawArrays(mode, start, count)
    }
}

pub fn draw_elements(mode: GLenum, count: GLsizei) {
    unsafe {
        log_draw!(
            "gl::DrawElements({}, count:{:?}, gl::UNSIGNED_INT, offset:{:?})",
            gl_draw_mode_enum_to_string(mode),,
            count,
            0
        );
        gl::DrawElements(
            mode,
            count,
            gl::UNSIGNED_INT,
            ptr::null_mut()
        );
    }
}

pub fn gl_attribute_enum_to_string(data_type: GLenum) -> &'static str () {
    match data_type {
        gl::FLOAT => "gl::FLOAT",
        gl::FLOAT_VEC2 => "gl::FLOAT_VEC2",
        gl::FLOAT_VEC3 => "gl::FLOAT_VEC3",
        gl::FLOAT_VEC4 => "gl::FLOAT_VEC4",
        gl::FLOAT_MAT2 => "gl::FLOAT_MAT2",
        gl::FLOAT_MAT3 => "gl::FLOAT_MAT3",
        gl::FLOAT_MAT4 => "gl::FLOAT_MAT4",
        gl::FLOAT_MAT2x3 => "gl::FLOAT_MAT2x3",
        gl::FLOAT_MAT2x4 => "gl::FLOAT_MAT2x4",
        gl::FLOAT_MAT3x2 => "gl::FLOAT_MAT3x2",
        gl::FLOAT_MAT3x4 => "gl::FLOAT_MAT3x4",
        gl::FLOAT_MAT4x2 => "gl::FLOAT_MAT4x2",
        gl::FLOAT_MAT4x3 => "gl::FLOAT_MAT4x3",
        gl::INT => "gl::INT",
        gl::INT_VEC2 => "gl::INT_VEC2",
        gl::INT_VEC3 => "gl::INT_VEC3",
        gl::INT_VEC4 => "gl::INT_VEC4",
        gl::UNSIGNED_INT => "gl::UNSIGNED_INT",
        gl::UNSIGNED_INT_VEC2 => "gl::UNSIGNED_INT_VEC2",
        gl::UNSIGNED_INT_VEC3 => "gl::UNSIGNED_INT_VEC3",
        gl::UNSIGNED_INT_VEC4 => "gl::UNSIGNED_INT_VEC4",
        gl::DOUBLE => "gl::DOUBLE",
        gl::DOUBLE_VEC2 => "gl::DOUBLE_VEC2",
        gl::DOUBLE_VEC3 => "gl::DOUBLE_VEC3",
        gl::DOUBLE_VEC4 => "gl::DOUBLE_VEC4",
        gl::DOUBLE_MAT2 => "gl::DOUBLE_MAT2",
        gl::DOUBLE_MAT3 => "gl::DOUBLE_MAT3",
        gl::DOUBLE_MAT4 => "gl::DOUBLE_MAT4",
        gl::DOUBLE_MAT2x3 => "gl::DOUBLE_MAT2x3",
        gl::DOUBLE_MAT2x4 => "gl::DOUBLE_MAT2x4",
        gl::DOUBLE_MAT3x2 => "gl::DOUBLE_MAT3x2",
        gl::DOUBLE_MAT3x4 => "gl::DOUBLE_MAT3x4",
        gl::DOUBLE_MAT4x2 => "gl::DOUBLE_MAT4x2",
        gl::DOUBLE_MAT4x3 => "gl::DOUBLE_MAT4x3",
        _ => panic!("Unknown gl attribute enum.")
    }
}

pub fn gl_draw_mode_enum_to_string(data_type: GLenum) -> &'static str () {
    match data_type {
        gl::POINTS => "gl::POINTS",
        gl::LINE_STRIP => "gl::LINE_STRIP",
        gl::LINE_LOOP => "gl::LINE_LOOP",
        gl::LINES => "gl::LINES",
        gl::LINE_STRIP_ADJACENCY => "gl::LINE_STRIP_ADJACENCY",
        gl::LINES_ADJACENCY => "gl::LINES_ADJACENCY",
        gl::TRIANGLE_STRIP => "gl::TRIANGLE_STRIP",
        gl::TRIANGLE_FAN => "gl::TRIANGLE_FAN",
        gl::TRIANGLES => "gl::TRIANGLES",
        gl::TRIANGLE_STRIP_ADJACENCY => "gl::TRIANGLE_STRIP_ADJACENCY",
        gl::TRIANGLES_ADJACENCY => "gl::TRIANGLES_ADJACENCY",
        gl::PATCHES => "gl::PATCHES",
        _ => panic!("Unknown gl draw mode enum.")
    }
}

pub fn gl_shader_type_enum_to_string(shader_type: GLenum) -> &'static str () {
    match shader_type {
        gl::VERTEX_SHADER => "gl::VERTEX_SHADER",
        gl::FRAGMENT_SHADER => "gl::FRAGMENT_SHADER",
        _ => "Unknown shader type"
    }
}
