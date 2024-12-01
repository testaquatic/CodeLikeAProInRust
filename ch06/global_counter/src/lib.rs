#[cfg(test)]
mod tests {
    use std::sync::{LazyLock, Mutex};

    // 표준 라이브러리에 있는 `[LazyLock](https://doc.rust-lang.org/beta/std/sync/struct.LazyLock.html)`이
    // lazy_static과 비슷한 역할을 하는 것 같아서 사용했다.
    // 자매품으로 스레드 안전성이 없는 `[LazyCell](https://doc.rust-lang.org/std/cell/struct.LazyCell.html)`이 있다.
    static MUTEX: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

    #[test]
    fn first_test() {
        let _guard = MUTEX.lock().expect("잠금을 얻지 못함");
        println!("첫 번째 테스트 진행 중");
    }

    #[test]
    fn second_test() {
        let lock = MUTEX.lock();
        let _guard = lock.expect("잠금을 얻지 못함");
        println!("두 번째 테스트 진행 중");
    }
}
