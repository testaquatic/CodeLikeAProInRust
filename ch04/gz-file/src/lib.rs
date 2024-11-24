use std::ffi::{c_char, c_int, c_uchar, c_uint, CString};


#[repr(C)]
struct GzFileState {
    have: c_uint,
    next: *mut c_uchar,
    pos: i64,
}

type GzFile = *mut GzFileState;

#[link(name = "z")]
extern "C" {
    fn gzopen(path: *const c_char, mode: *const c_char) -> GzFile;
    fn gzread(file: GzFile, buf: *mut c_uchar, len: c_uint) -> c_int;
    fn gzclose(file: GzFile) -> c_int;
    fn gzeof(file: GzFile) -> c_int;
}

pub fn read_gz_file(name: &str) -> String {
    let mut buffer = [0_u8; 0x1000];
    let mut contents = String::new();
    let c_name = CString::new(name).expect("CString failed");
    let c_mode = CString::new("r").expect("CString failed");
    unsafe {
        let file = gzopen(c_name.as_ptr(), c_mode.as_ptr());
        if file.is_null() {
            panic!("Couldn't read file: {}", std::io::Error::last_os_error());
        }
        while gzeof(file) == 0 {
            let bytes_read = gzread(file, buffer.as_mut_ptr(), (buffer.len() -1) as c_uint);
            let s = std::str::from_utf8(&buffer[..(bytes_read as usize)]).unwrap();
            contents.push_str(s);
        }
        gzclose(file);
    }
    contents
}