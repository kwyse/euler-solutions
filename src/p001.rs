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
///
/// Guass' method involves observing that each number in a sequence ending with
/// an even number can be paired. For example, for _n_ = 6:
///
/// ```text
/// 1 + 6 = 7
/// 2 + 5 = 7
/// 3 + 4 = 7
/// ```
///
/// There are three pairs (ie. 6 / 2), so the sum is (6 + 1) * (6 / 2), or
/// (_n_(_n_ + 1)) / 2.
///
/// For sequences that end in an odd number, pad the sequence with a zero:
///
/// ```text
/// 0 + 5 = 5
/// 1 + 4 = 5
/// 2 + 3 = 5
/// ```
///
/// Now the _n_ represents the sum of each pair, and the (_n_ + 1) / 2 represents
/// the number of pairs.
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
