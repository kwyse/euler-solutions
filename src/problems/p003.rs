#[allow(dead_code)]
fn p003() -> u64 {
    use prime::PrimeSequence;

    let target: u64 = 600851475143;
    let upper_bound = (target as f64).sqrt() as u64;

    PrimeSequence::new()
        .take_while(|&i| i < upper_bound)
        .filter(|i| target % i == 0)
        .max().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(6857, super::p003());
    }
}
