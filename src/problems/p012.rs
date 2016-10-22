#[allow(dead_code)]
fn p012() -> u64 {
    use prime;

    let num_divisors = |n: u64| {
        prime::factors(n).iter()
            .map(|factor| factor.exp)
            .map(|d| d + 1)
            .fold(1, |acc, d| acc * d)
    };

    ::TriangleSequence::new()
        .map(|i| (i, num_divisors(i)))
        .find(|&(_, ref num_divisors)| num_divisors > &500)
        .map(|(i, _)| i)
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(76_576_500, super::p012());
    }
}
