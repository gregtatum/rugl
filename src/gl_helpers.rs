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

pub unsafe fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    #[cfg(feature = "debug_draw")]
    println!("gl::CreateShader(shader_type:{})", match shader_type {
        gl::VERTEX_SHADER => "gl::VERTEX_SHADER",
        gl::FRAGMENT_SHADER => "gl::FRAGMENT_SHADER",
        _ => "Unknown shader type"
    });
    let shader = gl::CreateShader(shader_type);
    #[cfg(feature = "debug_draw")]
    println!("gl::CreateShader -> {:?}", shader);

    // Attempt to compile the shader
    let c_str = CString::new(source.as_bytes()).unwrap();
    #[cfg(feature = "debug_draw")]
    println!("\"{}\n\"", source);
    println!("gl::ShaderSource(shader:{:?}, count:{:?}, source:{:?}, ptr::null())", shader, 1, &c_str.as_ptr());
    gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());

    #[cfg(feature = "debug_draw")]
    println!("gl::CompileShader({:?})", shader);
    gl::CompileShader(shader);

    // Get the compile status
    let mut status: GLint = mem::uninitialized();
    #[cfg(feature = "debug_draw")]
    println!("gl::GetShaderiv(shader:{:?}, gl::COMPILE_STATUS, *status)", shader);
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    #[cfg(feature = "debug_draw")]
    println!("    status -> {}", status);

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

pub unsafe fn link_program(vertex_shader: &GLuint, fragment_shader: &GLuint) -> GLuint {
    let program = gl::CreateProgram();
    #[cfg(feature = "debug_draw")]
    println!("gl::CreateProgram() -> {:?}", program);
    #[cfg(feature = "debug_draw")]
    println!("gl::AttachShader(program:{:?}, vertex_shader:{:?})", program, *vertex_shader);
    gl::AttachShader(program, *vertex_shader);
    #[cfg(feature = "debug_draw")]
    println!("gl::AttachShader(program:{:?}, fragment_shader:{:?})", program, *fragment_shader);
    gl::AttachShader(program, *fragment_shader);
    #[cfg(feature = "debug_draw")]
    println!("gl::LinkProgram(program:{:?})", program);
    gl::LinkProgram(program);

    // Get the link status
    let mut status: GLint = mem::uninitialized();
    println!("gl::GetProgramiv(program:{:?}, gl::LINK_STATUS, &mut status)", program);
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
    #[cfg(feature = "debug_draw")]
    println!("    status -> {}", status);

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

pub unsafe fn create_buffer(vertex_data: &[GLfloat]) -> GLuint {
    // Create Vertex Array Object
    let mut vao: GLuint = mem::uninitialized();
    #[cfg(feature = "debug_draw")]
    println!("gl::GenVertexArrays(size:1, *vao)");
    gl::GenVertexArrays(1, &mut vao);
    #[cfg(feature = "debug_draw")]
    println!("    vao -> {}", vao);

    #[cfg(feature = "debug_draw")]
    println!("gl::BindVertexArray({})", vao);
    gl::BindVertexArray(vao);

    // Create a Vertex Buffer Object and copy the vertex data to it.
    let mut buffer: GLuint = mem::uninitialized();
    #[cfg(feature = "debug_draw")]
    println!("gl::GenBuffers(size:1, *buffer)");
    gl::GenBuffers(1, &mut buffer);
    #[cfg(feature = "debug_draw")]
    println!("    buffer -> {}", buffer);

    #[cfg(feature = "debug_draw")]
    println!("gl::BindBuffer(gl::ARRAY_BUFFER, buffer:{:?})", buffer);
    gl::BindBuffer(gl::ARRAY_BUFFER, buffer);

    let size = (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
    #[cfg(feature = "debug_draw")] {
        println!(
            "gl::BufferData(gl::ARRAY_BUFFER, size:{:?}, *data, gl::STATIC_DRAW)",
            size
        );
        println!("    vertex_data: {:?}", vertex_data);
        println!("    GLFloat size: {}", mem::size_of::<GLfloat>());
    }
    gl::BufferData(
        gl::ARRAY_BUFFER,
        size,
        mem::transmute(&vertex_data[0]),
        gl::STATIC_DRAW
    );

    // Make sure the gl state is clean.
    // #[cfg(feature = "debug_draw")]
    // println!("gl::BindBuffer(gl::ARRAY_BUFFER, buffer:0)");
    // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    vao
}

pub unsafe fn bind_attribute_buffer(
    vao: GLuint,
    attribute_info: &AttributeInfo
) {
    #[cfg(feature = "debug_draw")]
    println!("gl::BindVertexArray({})", vao);
    gl::BindVertexArray(vao);

    // Bind the buffer of data that's going in that slot.
    // #[cfg(feature = "debug_draw")]
    // println!("gl::BindBuffer(gl::ARRAY_BUFFER, {})", vbo);
    // gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

    // Enable the slot in the shader for this attribute.
    #[cfg(feature = "debug_draw")]
    println!("gl::EnableVertexAttribArray({})", attribute_info.index);
    gl::EnableVertexAttribArray(attribute_info.index);

    // Define how the pointers look up the information in the buffer.
    #[cfg(feature = "debug_draw")]
    println!(
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

    #[cfg(feature = "debug_draw")]
    println!(
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
    #[cfg(feature = "debug_draw")] {
        println!("    name -> {:?}", name);
        println!("    name_length -> {}", name_length);
        println!("    data_size -> {}", data_size);
        println!("    data_type -> {}", gl_attribute_enum_to_string(data_type));
    }

    let info = AttributeInfo {
        name: name,
        index: attribute_index as GLuint,
        data_type: get_attribute_type(data_type),
        data_size: get_attribute_type_size(data_type),
        type_enum: data_type
    };

    #[cfg(feature = "debug_draw")]
    println!("{:?}", info);

    info
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
        _ => panic!("Unknown gl enum returned")
    }
}
