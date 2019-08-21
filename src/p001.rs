//! Problem 1: Multiples of 3 and 5
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we
//! get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.

/// Sum all multiples of the input sequence up to a limit
pub fn sum_all_multiples(xs: &[u32], n: u32) -> u32 {
    let sum_multiples_for_x = |x| x * sum_to((n - 1) / x);

    let multiples = xs.iter().map(sum_multiples_for_x).sum::<u32>();
    let product_multiples = sum_multiples_for_x(&xs.iter().product::<u32>());
    multiples - product_multiples
}

fn sum_to(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiples_are_summed() {
        assert_eq!(sum_all_multiples(&[3, 5], 10), 23);
    }

    #[test]
    fn p001() {
        assert_eq!(sum_all_multiples(&[3, 5], 1_000), 233_168);
    }
}
