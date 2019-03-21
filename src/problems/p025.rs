//! Problem 25: 1000-digit Fibonacci number

use bigint::BigInt;
use fib::FibonacciSequence;

solve!(expecting_answer: 4782, with: || {
    FibonacciSequence(BigInt(vec![1]), BigInt(vec![1]))
        .enumerate()
        .find(|(_, n)| n.num_digits() >= 1000)
        .map(|(i, _)| i + 1)
        .unwrap_or(0)
});

mod fib {
    use super::bigint::BigInt;

    pub struct FibonacciSequence(pub BigInt, pub BigInt);

    impl Iterator for FibonacciSequence {
        type Item = BigInt;

        fn next(&mut self) -> Option<Self::Item> {
            let current = self.0.clone();
            self.0 = self.1.clone();
            self.1 = self.1.add(&current);

            Some(current)
        }
    }
}

mod bigint {
    const LIMIT: u128 = 1_000_000_000_000_000_000_000_000_000_000_000_000;

    #[derive(Clone)]
    pub struct BigInt(pub Vec<u128>);

    impl BigInt {
        pub fn add(&self, other: &Self) -> Self {
            let mut carry = 0;
            let mut result = Vec::new();
            for i in 0..self.0.len() {
                let sum = carry + self.0[i] + other.0.get(i).unwrap_or(&0);
                result.push(sum % LIMIT);
                carry = sum / LIMIT;
            }

            if carry > 0 {
                result.push(carry);
            }

            BigInt(result)
        }

        pub fn num_digits(&self) -> usize {
            self.0.iter().map(|&m| ((m as f64).log10() as usize) + 1).sum()
        }
    }
}
