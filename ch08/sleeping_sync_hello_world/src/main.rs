use std::{thread, time};

fn main() {
    let duration = time::Duration::from_secs(1);
    thread::sleep(duration);
    println!("Hello, world!");
}
