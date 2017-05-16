use super::gl::types::*;
use super::Primitive;
use super::gl_helpers::create_buffer;
use super::gl_helpers::create_buffer_u32;
use std::mem;
use std::slice;

// Take a reference to some vector data, and pretend to transfer it over to the gl state.
pub trait BufferableData { fn to_buffer(&self) -> GLuint; }

impl BufferableData for Vec<f32> {
    fn to_buffer(&self) -> GLuint {
        create_buffer(self)
    }
}

/**
 * For all the rest of the code, we have to unsafely transmute our Vec<[f32; N]>
 * from a Vector, to a reference to slice. The actual data that the Vec points
 * to in the heap is correctly laid out how we want to use it on the GL side.
 * The basic process is to pull a pointer to our data, unsafely transmute_copy
 * a pointer to it, removing the type from the compiler. Finally we pass that
 * into the foreign function interface to be consumed by the gl state machine.
 */

/**
 * TODO - Per jimb, consider moving the unsafe code:
 *        impl ReshapeSlice<f32> for [f32;3]
 *        and then `BufferableData` would just call `self.reshape`
 *        or ReshapeSlice<f32>::reshape(self) for Universal Function Call Syntax
 */

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

impl BufferableData for &'static [[f32; 3]] {
    fn to_buffer(&self) -> GLuint {
        unsafe {
            let data: &[f32] = slice::from_raw_parts(
                mem::transmute_copy::<*const [f32; 3], *mut f32>(&self.as_ptr()),
                self.len() * 3
            );
            create_buffer(&data)
        }
    }
}

/**
 * Enumerate options for buffers used as the elements in gl::DrawElements().
 */
pub trait BufferableElementsData {
    fn to_buffer(&self, &Primitive) -> GLuint;
    fn get_count(&self, &Primitive) -> GLint;
}

impl BufferableElementsData for Vec<u32> {
    fn to_buffer(&self, _: &Primitive) -> GLuint {
        create_buffer_u32(self)
    }

    fn get_count(&self, _: &Primitive) -> GLint {
        self.len() as i32
    }
}

impl BufferableElementsData for &'static [[u32; 2]] {
    fn to_buffer(&self, primitive: &Primitive) -> GLuint {
        match primitive {
            &Primitive::Triangles => {
                unsafe {
                    let data: &[u32] = slice::from_raw_parts(
                        mem::transmute_copy::<*const [u32; 2], *mut u32>(&self.as_ptr()),
                        self.len() * 2
                    );
                    create_buffer_u32(&data)
                }
            },
            _ => panic!("Trying to set element arrays that are not the valid type")
        }
    }

    fn get_count(&self, primitive: &Primitive) -> GLint {
        match primitive {
            &Primitive::Lines => 2 * (self.len() as i32),
            _ => panic!("Trying to get the count of an invalid combo of primitive and array type.")
        }
    }
}

impl BufferableElementsData for &'static [[u32; 3]] {
    fn to_buffer(&self, primitive: &Primitive) -> GLuint {
        match primitive {
            &Primitive::Triangles => {
                unsafe {
                    let data: &[u32] = slice::from_raw_parts(
                        mem::transmute_copy::<*const [u32; 3], *mut u32>(&self.as_ptr()),
                        self.len() * 3
                    );
                    create_buffer_u32(&data)
                }
            },
            _ => panic!("Trying to set element arrays that are not the valid type")
        }
    }

    fn get_count(&self, primitive: &Primitive) -> GLint {
        match primitive {
            &Primitive::Triangles => 3 * (self.len() as i32),
            _ => panic!("Trying to get the count of an invalid combo of primitive and array type.")
        }
    }
}
