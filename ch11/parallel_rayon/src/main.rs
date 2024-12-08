use std::{iter, time::Instant};

use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

fn generate_random_values(count: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let mut ret = vec![0; count];

    rng.fill(ret.as_mut_slice());

    ret
}

fn generate_random_strings(count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    iter::from_fn(|| Some(Alphanumeric.sample_string(&mut rng, 50000)))
        .take(count)
        .collect()
}

fn main() {
    let data = generate_random_values(100_000);
    let start = Instant::now();
    let _sum = data
        .iter()
        .map(|n| n.wrapping_mul(*n))
        .reduce(|a, b| a.wrapping_add(b));
    let finish = Instant::now() - start;
    println!(
        "Summing squares without rayon took {}s",
        finish.as_secs_f64()
    );

    let start = Instant::now();
    let _sum = data
        .par_iter()
        .map(|n| n.wrapping_mul(*n))
        .reduce(|| 0, |a, b| a.wrapping_add(b));
    let finish = Instant::now() - start;
    println!("Summing squares with rayon took {}s", finish.as_secs_f64());

    let re = Regex::new(r"catdog").unwrap();

    let data = generate_random_strings(500);
    let start = Instant::now();
    let _matches = data.iter().filter(|s| re.is_match(s)).collect::<Vec<_>>();
    let finish = Instant::now() - start;
    println!("Regex took {}s", finish.as_secs_f64());

    let start = Instant::now();
    let _matches = data
        .par_iter()
        .filter(|s| re.is_match(s))
        .collect::<Vec<_>>();
    let finish = Instant::now() - start;
    println!("Regex with rayon took {}s", finish.as_secs_f64());
}
