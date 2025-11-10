use std::{path::Path};

use gl::types::{GLenum, GLuint};

pub enum ShaderType {
   Vertex,
   Fragment,
}

impl ShaderType {
    pub fn unzip(self) -> GLenum {
        match self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

pub trait Shader {
    fn id(&self) -> GLuint;
    fn name(&self) -> &str;
    fn compile_from(name: &str, src: &Path) -> Self;
}

pub enum DrawMode {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
}

impl DrawMode {
    pub fn unzip(self) -> GLenum {
        match self {
            DrawMode::Points => gl::POINTS,
            DrawMode::Lines => gl::LINES,
            DrawMode::LineStrip => gl::LINE_STRIP,
            DrawMode::Triangles => gl::TRIANGLES,
            DrawMode::TriangleStrip => gl::TRIANGLE_STRIP,
        }
    }
}
