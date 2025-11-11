use crate::{
    shader::{
        fragment::FragmentShader,
        vertex::VertexShader,
        core::{Shader, DrawMode},
        base::*
    },
    constant::Type
};

pub struct Program {
    id: u32,
    vertex_shader: String,
    fragment_shader: String,
}

impl Program {
    pub fn link_from(vertex_shader: &VertexShader, fragment_shader: &FragmentShader) -> Self {
        let id = create_program();
        attach_shader(id, vertex_shader.id());
        attach_shader(id, fragment_shader.id());
        link_program(id);


        let check = get_programiv(id, gl::LINK_STATUS);
        if check == 0 {
            let info = get_program_info_log(id);
            println!("从顶点渲染器{}与片段渲染器{}构建渲染程序时发生错误：{}" ,vertex_shader.name(), fragment_shader.name(), info);
            panic!()
        }

        Self {
            id,
            vertex_shader: vertex_shader.name().to_string(),
            fragment_shader: fragment_shader.name().to_string(),
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn vertex_shader(&self) -> &str {
        &self.vertex_shader
    }
    pub fn fragment_shader(&self) -> &str {
        &self.fragment_shader
    }
    pub fn use_program(&self) {
        use_program(self.id());
    }
    pub fn draw_arrays(&mut self, mode: DrawMode, first: i32, count: i32) {
        draw_arrays(mode.unzip(), first, count);
    }
    pub fn draw_elements(&mut self, mode: DrawMode, count: i32, type_: Type, offset: Option<usize>) {
        draw_elements(mode.unzip(), count, type_.unzip(), offset);
    }
}
