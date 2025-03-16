//! # rustdoc-example.
//!
//! A simple project demonstrating the use of rustdoc with the function
//! [`mult`].
//!
//! # Example
//!
//! ```
//! use rustdoc_example::mult;
//!
//! assert_eq!(mult(10, 10), 100);
//! ```

#![warn(missing_docs)]

mod outer_module;

/// Returns the product of `a` and `b`
pub fn mult(a: u64, b: u64) -> u64 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = mult(2, 2);
        assert_eq!(result, 4);
    }
}
