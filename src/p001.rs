//! Problem 1: Multiples of 3 and 5
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we
//! get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.

/// Sums the multiples of a set of numbers up to a limit
///
/// This makes use of Guass' method of summing numbers up to a given bound. Since
/// the product of the input set of numbers is included in the sum twice, it must
/// be subtracted.
pub fn sum_of_multiples(ns: &[u32], limit: u32) -> u32 {
    let sum_divisible_by = |n| {
        let upper_bound = (limit - 1) / n;
        n * (upper_bound * (upper_bound + 1)) / 2
    };

    let sum_of_input = ns.iter().map(sum_divisible_by).sum::<u32>();
    let sum_of_input_product = sum_divisible_by(&ns.iter().product::<u32>());
    sum_of_input - sum_of_input_product
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
        assert_eq!(sum_of_multiples(&[3, 5], 1_000), 233_168);
    }
}
