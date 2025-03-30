use std::collections::HashMap;

#[allow(dead_code)]
fn fizzbuzz(n: i32) -> Vec<String> {
    let mut result = Vec::new();
    for i in 1..(n + 1) {
        if i % 15 == 0 {
            result.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            result.push("Fizz".to_string());
        } else if i % 5 == 0 {
            result.push("Buzz".to_string());
        } else {
            result.push(i.to_string());
        }
    }

    result
}

#[allow(dead_code)]
fn better_fizzbuzz(n: i32) -> Vec<String> {
    let mappings = HashMap::from([(3, "Fizz"), (5, "Buzz")]);
    let mut result = vec![String::new(); n as usize];
    let mut keys = mappings.keys().collect::<Vec<_>>();
    keys.sort();

    for i in 0..n {
        for key in keys.iter() {
            if (i + 1) % *key == 0 {
                result[i as usize].push_str(mappings.get(key).expect("couldn't fetch mapping"));
            }
        }

        if result[i as usize].is_empty() {
            result[i as usize] = (i + 1).to_string();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::vec;

    use proptest::proptest;

    use crate::{better_fizzbuzz, fizzbuzz};

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(3), vec!["1", "2", "Fizz"]);
        assert_eq!(fizzbuzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(
            fizzbuzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }

    #[test]
    fn test_better_fizzbuzz() {
        assert_eq!(better_fizzbuzz(3), vec!["1", "2", "Fizz"]);
        assert_eq!(better_fizzbuzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(
            better_fizzbuzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }

    proptest! {
        #[test]
        fn test_better_fizzbuzz_proptest(n in 1i32..=10000) {
            assert_eq!(fizzbuzz(n), better_fizzbuzz(n));
        }
    }
}
