//! Problem 23: Non-abundant sums

solve!(expecting_answer: 4_179_871, with: || {
    use std::collections::HashSet;

    let divisors = |n| (1..(n / 2 + 1))
        .filter(|m| n % m == 0)
        .collect::<Vec<_>>();

    let abundant_numbers = (1..=28_123)
        .filter(|&n| divisors(n).iter().sum::<u32>() > n)
        .collect::<HashSet<u32>>();

    (1..28_123).filter(|n: &u32| {
        !abundant_numbers.iter().any(|&m: &_|
            abundant_numbers.get(&(n.checked_sub(m).unwrap_or(0))).is_some()
        )
    }).sum::<u32>()
});
