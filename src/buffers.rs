use super::gl::types::*;
use super::gl_helpers::create_buffer;
use std::mem;
use std::slice;

// Take a reference to some vector data, and pretend to transfer it over to the gl state.
pub trait BufferableData { fn to_buffer(&self) -> GLuint; }

impl BufferableData for Vec<f32> {
    fn to_buffer(&self) -> GLuint {
        create_buffer(self)
    }
}

impl BufferableData for Vec<[f32; 2]> {
    fn to_buffer(&self) -> GLuint {
        unsafe {
            let data: &[f32] = slice::from_raw_parts(
                mem::transmute_copy::<*const [f32; 2], *mut f32>(&self.as_ptr()),
                self.len() * 2
            );
            create_buffer(data)
        }
    }
}

impl BufferableData for Vec<[f32; 3]> {
    fn to_buffer(&self) -> GLuint {
        unsafe {
            let vec: Vec<f32> = Vec::from_raw_parts(
                mem::transmute_copy::<*const [f32; 3], *mut f32>(&self.as_ptr()),
                self.len() * 3,
                self.len() * 3
            );
            create_buffer(&vec)
        }
    }
}

impl BufferableData for Vec<[f32; 4]> {
    fn to_buffer(&self) -> GLuint {
        unsafe {
            let vec: Vec<f32> = Vec::from_raw_parts(
                mem::transmute_copy::<*const [f32; 4], *mut f32>(&self.as_ptr()),
                self.len() * 4,
                self.len() * 4
            );
            create_buffer(&vec)
        }
    }
}

impl BufferableData for Vec<[f32; 9]> {
    fn to_buffer(&self) -> GLuint {
        unsafe {
            let vec: Vec<f32> = Vec::from_raw_parts(
                mem::transmute_copy::<*const [f32; 9], *mut f32>(&self.as_ptr()),
                self.len() * 9,
                self.len() * 9
            );
            create_buffer(&vec)
        }
    }
}

impl BufferableData for Vec<[f32; 12]> {
    fn to_buffer(&self) -> GLuint {
        unsafe {
            let vec: Vec<f32> = Vec::from_raw_parts(
                mem::transmute_copy::<*const [f32; 12], *mut f32>(&self.as_ptr()),
                self.len() * 12,
                self.len() * 12
            );
            create_buffer(&vec)
        }
    }
}

impl BufferableData for Vec<[f32; 16]> {
    fn to_buffer(&self) -> GLuint {
        unsafe {
            let vec: Vec<f32> = Vec::from_raw_parts(
                mem::transmute_copy::<*const [f32; 16], *mut f32>(&self.as_ptr()),
                self.len() * 16,
                self.len() * 16
            );
            create_buffer(&vec)
        }
    }
}
