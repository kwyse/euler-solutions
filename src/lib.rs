//! Abstractions over Project Euler problems

#![feature(step_by)]

pub use fib::FibonacciSequence;
pub use prime::PrimeSequence;

mod p001_010;

/// For dealing with a Fibonacci sequence
mod fib {
    use std::ops::Add;

    pub struct FibonacciSequence<T> {
        current: T,
        next: T,
    }

    impl<T: From<u8>> FibonacciSequence<T> {
        pub fn new() -> Self {
            FibonacciSequence {
                current: T::from(0),
                next: T::from(1)
            }
        }
    }

    impl<T> Iterator for FibonacciSequence<T>
        where T: Add<Output = T> + Copy {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            use std::mem;

            let new_next = self.current + self.next;
            self.current = mem::replace(&mut self.next, new_next);
            Some(self.current)
        }
    }
}

/// For dealing with prime numbers
mod prime {
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

    pub fn is_prime(num: &u64) -> bool {
        if *num == 2 || *num == 3 { true }
        else {
            let upper_bound = (*num as f64).sqrt() as u64 + 1;
            !(2..upper_bound).any(|i| num % i == 0)
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_fib_sequence() {
        let sequence: Vec<u64> = ::FibonacciSequence::new().take(6).collect();

        assert_eq!(&1, sequence.get(0).unwrap());
        assert_eq!(&1, sequence.get(1).unwrap());
        assert_eq!(&2, sequence.get(2).unwrap());
        assert_eq!(&3, sequence.get(3).unwrap());
        assert_eq!(&5, sequence.get(4).unwrap());
        assert_eq!(&8, sequence.get(5).unwrap());
    }

    #[test]
    fn test_is_prime() {
        use ::prime::is_prime;

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
        let sequence: Vec<u64> = ::PrimeSequence::new().take(10_001).collect();

        assert_eq!(&2, sequence.get(0).unwrap());
        assert_eq!(&3, sequence.get(1).unwrap());
        assert_eq!(&5, sequence.get(2).unwrap());
        assert_eq!(&104_743, sequence.get(10_000).unwrap());
    }
}
