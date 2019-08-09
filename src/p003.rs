//! Problem 3: Largest prime factor
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143?

pub fn largest_prime_factor(n: u64) -> u64 {
    *prime_factors(n).iter().max().unwrap_or(&0)
}

pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let limit = (n as f32).sqrt() as u64;

    let mut factors = Vec::new();
    let mut check_and_reduce = |m| {
        if n % m == 0 {
            factors.push(m);
        }

        while n % m == 0 {
            n /= m;
        }
    };

    check_and_reduce(2);
    for m in (3..limit).step_by(2) {
        check_and_reduce(m);
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_prime_factor_is_calculated() {
        assert_eq!(largest_prime_factor(13_195), 29);
    }

    #[test]
    fn prime_factors_are_calculated() {
        assert_eq!(prime_factors(13_195), [5, 7, 13, 29]);
    }

    #[test]
    fn p003() {
        assert_eq!(largest_prime_factor(600_851_475_143), 6_857);
    }
}
