use zlib_wrap::{zlib_compress, zlib_uncompress};

fn main() {
    let hello_world = "Hello, world!".as_bytes();
    let hello_world_compressed = zlib_compress(hello_world);
    let hello_world_uncompressed = zlib_uncompress(&hello_world_compressed, 100);
    assert_eq!(hello_world, hello_world_uncompressed);
    println!(
        "{}",
        String::from_utf8(hello_world_uncompressed).expect("Invalid characters")
    );
}
