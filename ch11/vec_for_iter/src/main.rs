use std::{time::Instant, vec};

fn main() {
    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    for i in big_vec {
        if i < 0 {
            println!("this never prints");
        }
    }

    println!("First loop took\t\t{}s", now.elapsed().as_secs_f32());

    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    big_vec.iter().for_each(|i| {
        if *i < 0 {
            println!("this never prints");
        }
    });
    println!("Second loop took\t{}s", now.elapsed().as_secs_f32());

    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    big_vec.into_iter().for_each(|i| {
        if i < 0 {
            println!("this never prints");
        }
    });
    println!("Third loop took\t\t{}s", now.elapsed().as_secs_f32());
}
