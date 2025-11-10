use gl::types::GLuint;

use crate::{
    buffer::{
        base::*,
        core::*,
        generator::Generator,
    },
    constant::{Boolean, Type},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vao {
    pub (in crate::buffer)id: GLuint,
}

impl Bind for Vao {
    fn bind(&self) {
        bind_vertex_arrays(self.id);
    }
    fn unbind(&self) {
        unbind_vertex_arrays();
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        delete_vertex_arrays(1, &[self.id]);
    }
}

impl IdObject for Vao {
    fn id(&self) -> GLuint {
        self.id
    }
    fn create(geneater: &mut Generator) -> Self {
        geneater.step();
        let mut id = geneater.max_id;
        gen_vertex_arrays(1, &mut id);
        Vao { id }
    }
}

impl Vao {
    pub fn upload(
        &mut self,
        index: u32,
        size: i32,
        type_: Type,
        normalized: Boolean,
        stride: i32,
        pointer: Option<usize>
    ) {
        vertex_attrib_pointer(index, size, type_.unzip(), normalized.unzip(), stride, pointer);
    }

    pub fn enable(index: u32) {
        enable_vertex_attrib_array(index);
    }

    pub fn disable(index: u32) {
        disable_vertex_attrib_array(index);
    }
}
