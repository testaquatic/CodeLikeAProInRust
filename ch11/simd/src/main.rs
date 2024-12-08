#![feature(portable_simd)]

use std::{simd::Simd, time::Instant};

fn initialize() -> ([u64; 64], [u64; 64]) {
    let mut a = [0; 64];
    let mut b = [0; 64];
    (0..64).for_each(|n| {
        a[n] = u64::try_from(n).unwrap();
        b[n] = u64::try_from(n + 1).unwrap();
    });
    (a, b)
}

fn main() {
    let (a, b) = initialize();

    let mut r = Vec::new();
    let now = Instant::now();
    for _ in 0..100_000 {
        let c = a.iter().zip(b).map(|(l, r)| l * r);
        let d = a.iter().zip(c.clone()).map(|(l, r)| l * r);
        let e = c.zip(d.clone()).map(|(l, r)| l * r);
        r = Vec::from_iter(e.zip(d).map(|(l, r)| l ^ r));
    }
    println!("Without SIMD took {}s", now.elapsed().as_secs_f32());

    let (a_vec, b_vec) = initialize();

    let a_vec = Simd::from(a_vec);
    let b_vec = Simd::from(b_vec);

    let mut r_simd = Vec::new();
    let now = Instant::now();
    for _ in 0..100_000 {
        let c_vec = a_vec * b_vec;
        let d_vec = a_vec * c_vec;
        let e_vec = c_vec * d_vec;
        let r_vec = e_vec ^ d_vec;
        r_simd = Vec::from(r_vec.as_array());
    }

    println!("With SIMD took {}s", now.elapsed().as_secs_f32());

    assert_eq!(r, r_simd);
}
