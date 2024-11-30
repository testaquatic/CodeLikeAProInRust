extern crate num_traits;
use num_traits::WrappingAdd;

pub fn add<T: WrappingAdd<Output = T>>(a: T, b: T) -> T {
    a.wrapping_add(&b)
}

#[cfg(test)]
mod tests {
    use proptest::proptest;

    use super::*;

    proptest! {
        #[test]
        fn test_add(a: i64, b: i64) {
            assert_eq!(add(a, b), a.wrapping_add(b));
        }
    }
}
