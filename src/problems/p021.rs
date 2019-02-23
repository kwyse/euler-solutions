//! Problem 21: Amicable numbers

solve!(expecting_answer: 31_626, with: || {
    use std::collections::HashSet;

    fn find_divisors(n: &u32) -> Vec<u32> {
        (1..(n / 2 + 1)).filter(|m| n % m == 0).collect()
    }

    fn is_amicable(a: &u32) -> bool {
        let b: u32 = find_divisors(a).iter().sum();
        let maybe_a: u32 = find_divisors(&b).iter().sum();
        *a == maybe_a && *a != b
    }

    let mut amicable_numbers = HashSet::new();

    for i in 1..10000 {
        if !amicable_numbers.contains(&i) {
            if is_amicable(&i) {
                amicable_numbers.insert(i);
                amicable_numbers.insert(find_divisors(&i).iter().sum::<u32>());
            }
        }
    }

    amicable_numbers.iter().sum::<u32>() as u128
});
