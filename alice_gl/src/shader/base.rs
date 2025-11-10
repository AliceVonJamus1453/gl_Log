use std::ffi::CString;

use gl::types::{GLenum, GLsizei, GLuint};

//这里开始是shader编译的部分
pub fn create_shader(type_: GLenum) -> GLuint {
    unsafe {
        gl::CreateShader(type_)
    }
}

pub fn shader_source(
    shader: u32,
    count: i32,
    string: String,
    length: Option<i32>
) {
    unsafe {
        gl::ShaderSource(
            shader as GLuint,
            count as GLsizei,
            &CString::new(string).unwrap().as_ptr(),
            match length {
                Some(len) => &len,
                None => std::ptr::null(),
            }
        );
    }
}

pub fn compile_shader(shader: u32) {
    unsafe {
        gl::CompileShader(shader as GLuint);
    }
}

pub fn get_shaderiv(shader: u32, pname: GLenum) -> i32 {
    let mut result = 0;
    unsafe {
        gl::GetShaderiv(shader as GLuint, pname, &mut result);
    }
    result
}

pub fn get_shader_info_log(shader: u32) -> String {
    let length = get_shaderiv(shader, gl::INFO_LOG_LENGTH);
    let mut buffer = vec![0; length as usize];
    unsafe {
        gl::GetShaderInfoLog(
            shader as GLuint,
            length,
            std::ptr::null_mut(),
            buffer.as_mut_ptr() as *mut i8
        );
    }
    String::from_utf8(buffer).unwrap()
}

//这里开始是shader链接的部分
pub fn create_program() -> GLuint {
    unsafe {
        gl::CreateProgram()
    }
}

pub fn attach_shader(program: u32, shader: u32) {
    unsafe {
        gl::AttachShader(program as GLuint, shader as GLuint);
    }
}

pub fn link_program(program: u32) {
    unsafe {
        gl::LinkProgram(program as GLuint);
    }
}

pub fn get_programiv(program: u32, pname: GLenum) -> i32 {
    let mut result = 0;
    unsafe {
        gl::GetProgramiv(program as GLuint, pname, &mut result);
    }
    result
}

pub fn get_program_info_log(program: u32) -> String {
    let length = get_programiv(program, gl::INFO_LOG_LENGTH);
    let mut buffer = vec![0; length as usize];
    unsafe {
        gl::GetProgramInfoLog(
            program as GLuint,
            length,
            std::ptr::null_mut(),
            buffer.as_mut_ptr() as *mut i8
        );
    }
    String::from_utf8(buffer).unwrap()
}

//从这里开始是绘制的部分
pub fn use_program(program: u32) {
    unsafe {
        gl::UseProgram(program as GLuint);
    }
}

pub fn draw_arrays(mode: GLenum, first: i32, count: i32) {
    unsafe {
        gl::DrawArrays(mode, first, count);
    }
}
