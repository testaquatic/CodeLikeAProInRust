use std::ffi::{c_char, CStr};

extern "C" {
    fn hello_world() -> *const c_char; 
}

pub fn call_hello_world() -> &'static str {
    unsafe {
        let hw = hello_world();
        CStr::from_ptr(hw).to_str().expect("String conversion failure")
    }
}