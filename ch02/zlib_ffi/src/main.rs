use std::ffi::{c_int, c_ulong};

#[link(name = "z")]
unsafe extern "C" {
    fn compress(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: c_ulong,
    ) -> c_int;
    fn compressBound(source_len: c_ulong) -> c_ulong;
    fn uncompress(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: c_ulong,
    ) -> c_int;
}

fn zlib_compress(source: &[u8]) -> Vec<u8> {
    unsafe {
        let source_len = source.len() as c_ulong;

        let mut dest_len = compressBound(source_len);
        let mut dest = Vec::with_capacity(dest_len as usize);

        compress(
            dest.as_mut_ptr(),
            &raw mut dest_len,
            source.as_ptr(),
            source_len,
        );
        dest.set_len(dest_len as usize);

        dest
    }
}

fn zlib_uncompress(source: &[u8], dest_len: usize) -> Vec<u8> {
    let source_len = source.len() as c_ulong;
    let source = source.as_ptr();
    let mut dest = Vec::with_capacity(dest_len);
    let mut dest_len = dest_len as c_ulong;
    unsafe {
        uncompress(dest.as_mut_ptr(), &raw mut dest_len, source, source_len);
        dest.set_len(dest_len as usize);
    }

    dest
}

fn main() {
    let hello_world = b"Hello World!";
    let hello_world_compressed = zlib_compress(hello_world);
    let hello_world_uncompressed = zlib_uncompress(&hello_world_compressed, 100);
    assert_eq!(hello_world, hello_world_uncompressed.as_slice());

    println!(
        "{}",
        String::from_utf8(hello_world_uncompressed).expect("Invalid chracters")
    );
}
