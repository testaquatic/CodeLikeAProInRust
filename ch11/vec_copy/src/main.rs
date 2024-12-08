use std::time::Instant;

fn main() {
    let big_vec_source = vec![0; 10_000_000];
    let mut big_vec_target = Vec::with_capacity(10_000_000);
    let now = Instant::now();
    big_vec_source
        .into_iter()
        .for_each(|i| big_vec_target.push(i));
    println!("Naive copy took {}s", now.elapsed().as_secs_f32());

    let big_vec_source = vec![0; 10_000_000];
    let mut big_vec_target = Vec::with_capacity(10_000_000);
    // 책의 코드는 컴파일러의 최적화의 영향을 받을 것 같아서 코드를 살짝 변경했다.
    unsafe {
        big_vec_target.set_len(10_000_000);
    }
    let now = Instant::now();
    big_vec_target.copy_from_slice(&big_vec_source);
    println!("Fast copy took {}s", now.elapsed().as_secs_f32());
    assert_eq!(big_vec_source, big_vec_target);

    let big_vec_source = vec![0; 10_000_000];
    let mut big_vec_target = Vec::with_capacity(10_000_000);
    let now = Instant::now();
    big_vec_target.extend_from_slice(&big_vec_source);
    println!("Extend took {}s", now.elapsed().as_secs_f32());
    assert_eq!(big_vec_source, big_vec_target);
}
