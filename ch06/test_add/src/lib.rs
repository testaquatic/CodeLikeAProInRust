use num_traits::WrappingAdd;

pub fn add<T: WrappingAdd<Output = T>>(a: T, b: T) -> T {
    a.wrapping_add(&b)
}

#[cfg(test)]
mod tests {
    use std::sync::{
        LazyLock, Mutex,
        atomic::{AtomicI32, Ordering},
    };

    use proptest::proptest;

    use crate::add;

    proptest! {
        #[test]
        fn test_add(a: i64, b: i64) {
            assert_eq!(add(a, b), a.wrapping_add(b));
        }
    }

    // 간단하게 이러면 되지 않을까
    static COUNT: AtomicI32 = AtomicI32::new(0);

    #[test]
    fn test_count() {
        COUNT.fetch_add(1, Ordering::SeqCst);
    }

    static MUTEX: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

    #[test]
    fn first_test() {
        let _guard = MUTEX.lock().expect("잠금을 얻지 못함");
        println!("첫 번째 테스트 진행 중");
    }

    #[test]
    fn second_test() {
        let _guard = MUTEX.lock().expect("잠금을 얻지 못함");
        println!("두 번째 테스트 진행 중");
    }
}
