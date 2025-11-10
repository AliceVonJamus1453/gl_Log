use std::{fs, path::Path};

use gl::types::GLuint;
use crate::shader::{
    base::*, core::{Shader, ShaderType}
};

pub struct VertexShader {
   name: String,
   id: GLuint
}

impl Shader for VertexShader {
    fn id(&self) -> GLuint {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn compile_from(name: &str, src: &Path) -> Self {
        let check = src.extension().unwrap();
        if check == "glsl" {
            eprintln!("Warning:问题出现于从{}构建{}时，顶点着色器拓展名最好使用.vert或.vs", src.display(), name);
        }
        else if check != "vert" && check != "vs" {
            panic!("从{}构建顶点着色器{}时发生错误：不支持的扩展名", src.display(), name);
        }

        let shader = create_shader(ShaderType::Vertex.unzip());
        let code = fs::read_to_string(src).unwrap();
        shader_source(shader, 1, code, None);
        compile_shader(shader);

        let check = get_shaderiv(shader, gl::COMPILE_STATUS);
        if check == 0 {
            panic!("从{}构建顶点着色器{}时发生错误：{}", src.display(), name, get_shader_info_log(shader));
        }
        VertexShader { name: name.to_string(), id: shader }
    }
}
