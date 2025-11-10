use gl::types::{GLboolean, GLenum};

pub enum Usage {
    StaticDraw,
}

impl Usage {
    pub fn unzip(self) -> GLenum {
        match self {
            Usage::StaticDraw => gl::STATIC_DRAW,
        }
    }
}

pub enum Type {
    Float,
}

impl Type {
    pub fn unzip(self) -> GLenum {
        match self {
            Type::Float => gl::FLOAT,
        }
    }
}

pub enum Boolean {
    True,
    False,
}

impl Boolean {
    pub fn unzip(self) -> GLboolean{
        match self {
            Boolean::True => gl::TRUE,
            Boolean::False => gl::FALSE,
        }
    }
}
