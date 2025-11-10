use gl::types::{GLenum, GLint, GLsizei, GLsizeiptr, GLuint, GLboolean, GLvoid};

use crate::constant::Usage;

/* 这里往下是Buffer的内容 */
pub fn gen_buffers(n: i32, buffers: &mut u32) {
    unsafe {
        gl::GenBuffers(n as GLsizei, buffers);
    }
}

pub fn bind_buffer(target: GLenum, buffer: u32) {
    unsafe {
        gl::BindBuffer(target, buffer as GLuint);
    }
}

pub fn unbind_buffer(target: GLenum) {
    unsafe {
        gl::BindBuffer(target, 0);
    }
}

pub fn delete_buffers(n: i32, buffers: &[u32]) {
    unsafe {
        gl::DeleteBuffers(n as GLsizei, buffers.as_ptr());
    }
}

pub fn buffer_data(target: GLenum, data: &[f32], usage: Usage) {
    unsafe {
        let size = (data.len() * std::mem::size_of::<f32>()) as GLsizeiptr;
        gl::BufferData(
            target,
            size,
            data.as_ptr() as *const GLvoid,
            usage.unzip()
        );
    }
}

/* 这里往下是VAO的内容 */
pub fn gen_vertex_arrays(n: i32, arrays: &mut u32) {
    unsafe {
        gl::GenVertexArrays(n as GLsizei, arrays);
    }
}

pub fn bind_vertex_arrays(array: u32) {
    unsafe {
        gl::BindVertexArray(array as GLuint);
    }
}

pub fn unbind_vertex_arrays() {
    unsafe {
        gl::BindVertexArray(0);
    }
}

pub fn delete_vertex_arrays(n: i32, arrays: &[u32]) {
    unsafe {
        gl::DeleteVertexArrays(n as GLsizei, arrays.as_ptr());
    }
}

pub fn vertex_attrib_pointer(
    index: u32,
    size: i32,
    type_: GLenum,
    normalized: GLboolean,
    stride: i32,
    pointer: Option<usize>
) {
    unsafe {
        gl::VertexAttribPointer(
            index as GLuint,
            size as GLint,
            type_,
            normalized,
            stride as GLsizei,
            {
                if let Some(ptr) = pointer {
                    ptr as *const GLvoid
                } else {
                    std::ptr::null()
                }
            }
        );
    }
}

pub fn enable_vertex_attrib_array(index: u32) {
    unsafe {
        gl::EnableVertexAttribArray(index as GLuint);
    }
}

pub fn disable_vertex_attrib_array(index: u32) {
    unsafe {
        gl::DisableVertexAttribArray(index as GLuint);
    }
}
