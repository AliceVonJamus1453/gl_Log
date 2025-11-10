use gl::types::{GLenum, GLuint};
use crate::{
    buffer::{
        generator::Generator,
    },
    constant::Usage,
};

pub enum BufferType {
    VBO,
}

impl BufferType {
    pub fn unzip(self) -> GLenum {
        match self {
            BufferType::VBO => gl::ARRAY_BUFFER,
        }
    }
}

pub trait Bind {
    fn bind(&self);
    fn unbind(&self);
}

pub trait Buffer {
    fn upload(&mut self, data: &[f32], usage: Usage);
}

pub trait IdObject {
    fn id(&self) -> GLuint;
    fn create(geneater: &mut Generator) -> Self;
}
