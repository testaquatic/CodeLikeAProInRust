use std::collections::HashMap;

struct FizzBuzz {
    fizz_count: i32,
    buzz_count: i32,
    count: i32,
    max_count: i32,
}

impl FizzBuzz {
    fn new(n: i32) -> Self {
        FizzBuzz {
            fizz_count: 0,
            buzz_count: 0,
            count: 0,
            max_count: n,
        }
    }
}

impl Iterator for FizzBuzz {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = None;
        self.count += 1;
        if self.count > self.max_count {
            return ret;
        }

        self.fizz_count += 1;
        if self.fizz_count == 3 {
            self.fizz_count = 0;
            ret = Some("Fizz".into());
        }
        self.buzz_count += 1;
        if self.buzz_count == 5 {
            self.buzz_count = 0;
            ret = ret
                .map(|mut fizz| {
                    fizz.push_str("Buzz");
                    fizz
                })
                .or(Some("Buzz".to_string()));
        }

        ret.or(Some(format!("{}", self.count)))
    }
}

pub fn iter_fizzbuzz(n: i32) -> Vec<String> {
    FizzBuzz::new(n).collect()
}

pub fn better_fizzbuz(n: i32) -> Vec<String> {
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
    use proptest::proptest;

    use crate::{better_fizzbuz, iter_fizzbuzz};

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(iter_fizzbuzz(3), vec!["1", "2", "Fizz"]);
        assert_eq!(iter_fizzbuzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(
            iter_fizzbuzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }

    proptest! {
        #[test]
        fn test_better_fizzbuzz_protest(n in 1i32..=10000) {
            assert_eq!(iter_fizzbuzz(n), better_fizzbuz(n));
        }
    }
}
