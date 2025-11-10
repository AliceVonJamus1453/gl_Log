use gl::types::{GLint, GLsizei};

pub fn viewport(x: i32, y: i32, width: u32, heigth: u32) {
    unsafe { gl::Viewport(x as GLint, y as GLint, width as GLsizei, heigth as GLsizei) }
}

pub fn clearcolor(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        gl::ClearColor(red, green, blue, alpha);
    }
}

pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
