extern crate proc_macro;
use alice_gl_macro_uniform::{
    make_uniform_func,
    make_uniform_func_vec,
    make_uniform_func_mat,
    impl_uniform_for_type,
};


make_uniform_func!();
make_uniform_func_vec!();
make_uniform_func_mat!();

pub fn get_uniform_location(program: u32, name: &str) -> i32 {
    unsafe {
        gl::GetUniformLocation(program, name.as_ptr() as *const i8)
    }
}

pub trait UniformType {}
impl_uniform_for_type!();
