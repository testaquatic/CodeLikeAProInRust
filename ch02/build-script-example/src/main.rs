use std::ffi::{CStr, c_char};

unsafe extern "C" {
    unsafe fn hello_world() -> *const c_char;
}

fn call_hello_world() -> &'static str {
    unsafe {
        let c_str = hello_world();
        CStr::from_ptr(c_str)
            .to_str()
            .expect("String conversion failure")
    }
}

fn main() {
    println!("{}", call_hello_world());
}
