use crate::buffer::{
    base::*,
    core::*,
    generator::*
};
use crate::constant::Usage;
use gl::types::GLuint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ebo {
    id: GLuint,
}

impl Bind for Ebo {
    fn bind(&self) {
        bind_buffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
    }
    fn unbind() {
        bind_buffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }
}

impl Drop for Ebo {
    fn drop(&mut self) {
        delete_buffers(1, &[self.id]);
    }
}


impl Buffer for Ebo {
    type DataType = u32;
    fn upload(&mut self, data: &[Self::DataType], usage: Usage) {
        buffer_data(gl::ELEMENT_ARRAY_BUFFER, data, usage);
    }
}

impl IdObject for Ebo {
    fn id(&self) -> GLuint {
        self.id
    }
    fn create(geneater: &mut Generator) -> Self {
        geneater.step();
        let mut id = geneater.max_id;
        gen_buffers(1, &mut id);
        Self { id }
    }
}
