//! Problem 20: Factorial digit sum

solve!(expecting_answer: 648, with: || {
    use num::{self, BigUint};

    num::range_inclusive(BigUint::new(vec![2]), BigUint::new(vec![100]))
        .fold(BigUint::new(vec![1]), |acc, x| acc * x)
        .to_string().chars().filter_map(|c| c.to_digit(10)).sum::<u32>() as u128
});
