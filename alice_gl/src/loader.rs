use std::ffi::c_void;

pub fn load_with(loadfn: impl FnMut(&str) -> *const c_void) {
    gl::load_with(loadfn);
}
