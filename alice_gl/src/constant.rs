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
    Double,
    Int,
    UInt,
}

impl Type {
    pub fn unzip(self) -> GLenum {
        match self {
            Type::Float => gl::FLOAT,
            Type::Double => gl::DOUBLE,
            Type::Int => gl::INT,
            Type::UInt => gl::UNSIGNED_INT,
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
