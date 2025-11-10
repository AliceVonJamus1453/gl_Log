use crate::buffer::{
    base::*,
    core::*,
    generator::*
};
use crate::constant::Usage;
use gl::types::GLuint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vbo {
    pub (in crate::buffer)id: GLuint,
}

impl Bind for Vbo {
    fn bind(&self) {
        bind_buffer(gl::ARRAY_BUFFER, self.id);
    }
    fn unbind(&self) {
        unbind_buffer(gl::ARRAY_BUFFER);
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        delete_buffers(1, &[self.id]);
    }
}


impl Buffer for Vbo {
    fn upload(&mut self, data: &[f32], usage: Usage) {
        buffer_data(gl::ARRAY_BUFFER, data, usage);
    }
}

impl IdObject for Vbo {
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
