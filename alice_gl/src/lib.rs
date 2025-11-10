use std::os::raw::c_void;

pub mod shader;
pub mod buffer;
pub mod constant;
pub mod window;

pub fn load_with(loadfn: impl FnMut(&str) -> *const c_void) {
    gl::load_with(loadfn);
}
