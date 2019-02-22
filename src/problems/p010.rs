//! Problem 10: Summation of primes

solve!(expecting_answer: 142_913_828_922, with: || {
    const limit: usize = 2_000_000;

    let idx = |i| (i - 3) / 2;
    let mut sieve = [true; (limit - 2) / 2];
    for n in (3..(limit as f64).sqrt() as usize + 1).step_by(2) {
        if sieve[idx(n)] {
            for m in (n * n..limit).step_by(n * 2) {
                sieve[idx(m)] = false;
            }
        }
    }

    2 + (0..sieve.len())
        .filter(|&i| sieve[i])
        .map(|i| (i * 2) + 3)
        .sum::<usize>() as u128
});
