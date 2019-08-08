//! Problem 1: Multiples of 3 and 5
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we
//! get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.

pub fn sum_of_multiples(ns: &[u32], limit: u32) -> u32 {
    (1..limit)
        .filter(|m| ns.iter().map(|n| m % n == 0).any(|m| m))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiples_are_summed() {
        assert_eq!(sum_of_multiples(&[3, 5], 10), 23);
    }

    #[test]
    fn p001() {
        assert_eq!(sum_of_multiples(&[3, 5], 1000), 233_168);
    }
}
