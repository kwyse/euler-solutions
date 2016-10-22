//! For dealing with prime numbers

/// Represents an iterative sequence of prime numbers
pub struct PrimeSequence {
    current: u64,
}

impl PrimeSequence {
    pub fn new() -> Self {
        PrimeSequence { current: 1 }
    }
}

impl Iterator for PrimeSequence {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let not_prime = |i: &u64| !is_prime(i);
        self.current = (self.current + 1..).skip_while(not_prime).next().unwrap();
        Some(self.current)
    }
}

/// Checks if a number is prime or not
pub fn is_prime(num: &u64) -> bool {
    if *num == 2 || *num == 3 { true }
    else {
        let upper_bound = (*num as f64).sqrt() as u64 + 1;
        !(2..upper_bound).any(|i| num % i == 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Factor<T> {
    pub base: T,
    pub exp: T,
}

/// Returns the prime factors of a number
pub fn factors(n: u64) -> Vec<Factor<u64>> {
    let base_factors = base_factors(n);
    let mut factors: Vec<Factor<u64>> = Vec::new();

    let mut i = 0;
    while i < base_factors.len() {
        let mut exp = 1;
        let mut i2 = i + 1;
        while let Some(&p2) = base_factors.get(i2) {
            if base_factors[i] == p2 {
                exp += 1;
                i2 += 1;
            } else {
                break;
            }
        }

        factors.push(Factor { base: base_factors[i] as u64, exp: exp });
        i += exp as usize;
    }

    factors
}

fn base_factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    for i in 3..(n as f64).sqrt() as u64 + 1 {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
    }

    if n > 2 { factors.push(n); }
    factors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(&2), true);
        assert_eq!(is_prime(&3), true);
        assert_eq!(is_prime(&4), false);
        assert_eq!(is_prime(&5), true);
        assert_eq!(is_prime(&8), false);
        assert_eq!(is_prime(&13), true);
        assert_eq!(is_prime(&27), false);
    }

    #[test]
    fn test_prime_sequence() {
        let sequence: Vec<u64> = PrimeSequence::new().take(10_001).collect();

        assert_eq!(&2, sequence.get(0).unwrap());
        assert_eq!(&3, sequence.get(1).unwrap());
        assert_eq!(&5, sequence.get(2).unwrap());
        assert_eq!(&104_743, sequence.get(10_000).unwrap());
    }

    #[test]
    fn test_factors() {
        let mut facts: Vec<Factor<u64>> = vec![Factor { base: 3, exp: 1 }];
        assert_eq!(facts, factors(3));

        facts = vec![Factor { base: 2, exp: 2 }, Factor { base: 7, exp: 1 }];
        assert_eq!(facts, factors(28));
    }
}
