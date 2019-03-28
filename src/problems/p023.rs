//! Problem 23: Non-abundant sums

fn solution() -> usize {
    const LIMIT: usize = 28_123;

    let abundants = (1..=LIMIT).filter(is_abundant).collect::<Vec<_>>();
    let mut should_include_in_sum = [true; LIMIT + 1];

    for (i, a) in abundants.iter().enumerate() {
        for b in &abundants[i..] {
            if a + b <= LIMIT {
                should_include_in_sum[a + b] = false
            }
        }
    }

    (1..=LIMIT).filter(|&i| should_include_in_sum[i]).sum()
}

fn is_abundant(n: &usize) -> bool {
    let divisors = (1..(*n as f32).sqrt() as usize + 1)
        .filter(move |m| n % m == 0)
        .collect::<Vec<_>>();

    let mut sum = divisors.iter().sum::<usize>();
    for divisor in divisors.iter().skip(1).rev().skip_while(|&&m| m * m == *n) {
        sum += n / divisor;
    }

    sum > *n
}

#[test]
fn test() {
    assert_eq!(solution(), 4_179_871);
}
