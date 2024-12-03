use quicksort::Quicksort;
use std::{env, num::ParseIntError};

fn main() -> Result<(), ParseIntError> {
    let mut values = env::args()
        .skip(1)
        .map(|s| s.parse())
        .collect::<Result<Vec<i64>, _>>()?;

    values.quicksort();

    println!("{values:?}");

    Ok(())
}
