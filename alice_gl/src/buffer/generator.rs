use gl::types::GLuint;

use crate::buffer::core::IdObject;

pub struct Generator {
    pub(in crate::buffer) max_id: GLuint,
}

impl Generator {
    pub fn new() -> Self {
        Generator { max_id: 0 }
    }
    pub fn step(&mut self) {
        self.max_id += 1;
    }
    pub fn set_start(&mut self, start: impl IdObject) {
        self.max_id = start.id();
    }
}
