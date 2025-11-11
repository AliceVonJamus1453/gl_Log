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
    fn unbind();
}

pub trait Buffer {
    type DataType;
    fn upload(&mut self, data: &[Self::DataType], usage: Usage);
}

pub trait IdObject {
    fn id(&self) -> GLuint;
    fn create(geneater: &mut Generator) -> Self;
}
