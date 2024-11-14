fn main() {
    println!("cargo:rebuild-if-changed=src/hello_world.c");
    cc::Build::new()
        .file("src/hello_world.c")
        .compile("hello_world");
}