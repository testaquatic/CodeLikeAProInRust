use std::env;

use quicksort::Quicksort;

fn main() {
    let mut values = env::args()
        .skip(1)
        .map(|s| s.parse::<i64>().expect(&format!("{s}: bad input: ")))
        .collect::<Vec<_>>();

    values.quicksort();

    println!("{values:?}");
}
