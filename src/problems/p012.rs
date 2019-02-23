//! Problem 12: Highly divisible triangular number

solve!(expecting_answer: 76_576_500, with: || {
    struct TriangleSequence(u32);
    impl Iterator for TriangleSequence {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            self.0 += 1;
            Some(self.0 * (self.0 + 1) / 2)
        }
    }

    let primes = {
        let mut sieve = [true; 1000];
        for i in 2..(1000 as f64).sqrt() as usize + 1 {
            if sieve[i] {
                for j in (i * 2..1000).step_by(2) { sieve[j] = false }
            }
        }

        (2..sieve.len()).filter(|&i| sieve[i])
            .map(|i| i as u32).collect::<Vec<_>>()
    };

    let num_factors = |mut n| {
        let mut exponents = Vec::new();
        for prime in &primes {
            let mut count = 0;
            while n % prime == 0 {
                n /= prime;
                count += 1;
            }

            exponents.push(count);
            if n == 1 { break }
        }

        exponents.iter().map(|&n| n + 1).product::<u32>()
    };

    TriangleSequence(0).find(|&n| num_factors(n) > 500).unwrap_or(0) as u128
});
