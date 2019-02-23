//! Problem 10: Summation of primes

solve!(expecting_answer: 142_913_828_922, with: || {
    const LIMIT: usize = 2_000_000;

    let idx = |i| (i - 3) / 2;
    let mut sieve = [true; (LIMIT - 2) / 2];
    for n in (3..(LIMIT as f64).sqrt() as usize + 1).step_by(2) {
        if sieve[idx(n)] {
            for m in (n * n..LIMIT).step_by(n * 2) {
                sieve[idx(m)] = false;
            }
        }
    }

    2 + (0..sieve.len())
        .filter(|&i| sieve[i])
        .map(|i| (i * 2) + 3)
        .sum::<usize>() as u128
});
