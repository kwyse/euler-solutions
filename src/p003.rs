//! Problem 3: Largest prime factor
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143?

/// Finds the largest prime factor of a number
pub fn largest_prime_factor(n: u64) -> Option<u64> {
    prime_factors(n).last().copied()
}

/// Calculates the prime factors of a number
///
/// A number _n_ can have at most one prime factor greater than sqrt(_n_). We
/// iterate up to this limit, reducing _n_ each time we encounter a factor, and
/// only check odd numbers since _2_ is the only even prime number.
///
/// If _n_ is not _one_ after the last iteration, the last-checked factor
/// exceeded sqrt(_n_) and _n_ is the last remaining prime factor of the input.
pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut check_and_reduce = |mut p, q| {
        if p % q == 0 {
            factors.push(q);
        }

        while p % q == 0 {
            p /= q;
        }

        p
    };

    check_and_reduce(n, 2);

    let mut factor = 3;
    let mut limit = (n as f32).sqrt() as u64;

    while n > 1 && factor <= limit {
        n = check_and_reduce(n, factor);

        factor += 2;
        limit = (n as f32).sqrt() as u64;

    }

    if n != 1 {
        factors.push(n);
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_prime_factor_is_calculated() {
        assert_eq!(largest_prime_factor(13_195), Some(29));
    }

    #[test]
    fn prime_factors_are_calculated() {
        assert_eq!(prime_factors(13_195), [5, 7, 13, 29]);
    }

    #[test]
    fn p003() {
        assert_eq!(largest_prime_factor(600_851_475_143), Some(6_857));
    }
}
