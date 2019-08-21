//! Problem 5: Smallest multiple
//!
//! 2520 is the smallest number that can be divided by each of the numbers from 1
//! to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the
//! numbers from 1 to 20?

/// Finds the lowest common multiple of all numbers up to _n_ inclusive
///
/// For all numbers to _n_ inclusive, calculates the lowest common multiple of
/// each window of two numbers. This is done by dividing their product by their
/// greatest common divisor. The lowest common multiple of the entire set is
/// the LCM of each window's LCM and the rolling LCM value.
pub fn lcm(n: u32) -> u32 {
    let mut lcm = 1;
    let numbers_to_n = (1..=n).collect::<Vec<_>>();

    for window in numbers_to_n.as_slice().windows(2) {
        let (a, b) = (window[0], window[1]);
        let window_lcm = (a * b) / gcd(a, b);
        lcm = (lcm * window_lcm) / gcd(lcm, window_lcm);
    }

    lcm
}

/// Finds the product of the minimal set of prime factors up to _n_ inclusive
///
/// For each prime number up to _n_, splits the set between primes less than the
/// square root of _n_ and primes that are greater. For those that are less, they
/// will occur more than once, so find their number of occurrences when
/// calculating their product. Multiply this by the product of the remaining
/// prime factors.
pub fn product_of_minimal_prime_factors(n: u32) -> u32 {
    let primes = primes(n);
    let under_sqrt = |m: &&u32| **m <= f64::from(n).sqrt() as u32;

    let single_factors: u32 = primes.iter().skip_while(under_sqrt).product();
    let repeated_factors: u32 = primes
        .iter()
        .take_while(under_sqrt)
        .map(|&m| m.pow(find_occurrences_of_prime_factor(n, m)))
        .product();

    single_factors * repeated_factors
}

/// Calculates the greatest common divisor
///
/// Uses the Euclidean algorithm.
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

/// Calculates number of times a prime factor occures
///
/// Given a number _n_, there is a number that is the smallest multiple of all
/// numbers from _1_ to _n_. This number is the product of some prime factors.
/// The number of times such a prime factor occurs in the product can be found
/// dividing the log of _n_ by the log of the factor, and flooring the result.
fn find_occurrences_of_prime_factor(n: u32, factor: u32) -> u32 {
    (f64::from(n).log10() / f64::from(factor).log10()) as u32
}

/// Finds the prime numbers up to and including _n_
///
/// Uses the Sieve of Eratosthenes.
fn primes(limit: u32) -> Vec<u32> {
    let mut primes = vec![true; limit as usize - 1];

    for i in 2..f64::from(limit).sqrt() as usize + 2 {
        if primes[i - 2] {
            for j in (i * 2..=limit as usize).step_by(i) {
                primes[j - 2] = false;
            }
        }
    }

    primes
        .iter()
        .enumerate()
        .filter(|(_, &b)| b)
        .map(|(a, _)| a as u32 + 2)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowest_common_multiple_is_calculated() {
        assert_eq!(lcm(2), 2);
        assert_eq!(lcm(3), 6);
        assert_eq!(lcm(4), 12);
        assert_eq!(lcm(5), 60);
        assert_eq!(lcm(6), 60);
        assert_eq!(lcm(10), 2_520);
    }

    #[test]
    fn product_of_minimal_prime_factors_is_calculated() {
        assert_eq!(product_of_minimal_prime_factors(2), 2);
        assert_eq!(product_of_minimal_prime_factors(3), 6);
        assert_eq!(product_of_minimal_prime_factors(4), 12);
        assert_eq!(product_of_minimal_prime_factors(5), 60);
        assert_eq!(product_of_minimal_prime_factors(6), 60);
        assert_eq!(product_of_minimal_prime_factors(10), 2_520);
    }

    #[test]
    fn greatest_common_divisor_is_calculated() {
        assert_eq!(gcd(9, 12), 3);
    }

    #[test]
    fn occurrences_of_prime_factor_is_calculated() {
        assert_eq!(find_occurrences_of_prime_factor(20, 2), 4);
        assert_eq!(find_occurrences_of_prime_factor(20, 3), 2);
        assert_eq!(find_occurrences_of_prime_factor(20, 5), 1);
        assert_eq!(find_occurrences_of_prime_factor(20, 7), 1);
    }

    #[test]
    fn primes_are_calculated() {
        assert_eq!(primes(20), [2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn p005() {
        assert_eq!(product_of_minimal_prime_factors(20), 232_792_560);
    }
}
