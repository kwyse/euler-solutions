//! *Problem 20 - Factorial digit sum*
//!
//! _n_! means _n_ × (_n_ − -1) × ... × 3 × 2 × 1
//!
//! For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
//! and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//!
//! Find the sum of the digits in the number 100!

use num::{self, BigUint};

#[test]
fn test() {
    assert_eq!(problem(), 648);
}

fn problem() -> u32 {
    num::range_inclusive(BigUint::new(vec![2]), BigUint::new(vec![100]))
        .fold(BigUint::new(vec![1]), |acc, x| acc * x)
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}
