//! Amicable numbers

use std::collections::HashSet;

pub fn p021(limit: u32) -> u32 {
    let mut amicable_numbers = HashSet::new();

    for i in 1..limit {
        if !amicable_numbers.contains(&i) {
            let sum_of_divisors: u32 = find_divisors(&i).iter().sum();
            if is_amicable_pair(&i, &sum_of_divisors) {
                amicable_numbers.insert(i);
                amicable_numbers.insert(sum_of_divisors);
            }
        }
    }

    amicable_numbers.iter().sum()
}

fn find_divisors(n: &u32) -> Vec<u32> {
    (1..(n / 2 + 1)).filter(|m| n % m == 0).collect()
}

fn is_amicable_pair(a: &u32, b: &u32) -> bool {
    let sum_a: u32 = find_divisors(a).iter().sum();
    let sum_b: u32 = find_divisors(b).iter().sum();
    sum_a == *b && sum_b == *a && *a != *b
}

#[test]
fn sample() {
    assert_eq!(find_divisors(&220), &[1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110]);
    assert_eq!(find_divisors(&284), &[1, 2, 4, 71, 142]);

    assert!(is_amicable_pair(&220, &284));
}

#[test]
fn problem() {
    assert_eq!(p021(10_000), 100);
}
