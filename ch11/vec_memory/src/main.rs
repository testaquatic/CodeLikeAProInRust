fn main() {
    let mut empty_vec = Vec::new();

    (0..10).for_each(|v| {
        println!(
            "empty_vec has {} elements with capacity {}",
            empty_vec.len(),
            empty_vec.capacity()
        );
        empty_vec.push(v);
    });
}
