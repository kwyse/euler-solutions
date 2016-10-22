#[allow(dead_code)]
fn p010() -> u64 {
    use prime::PrimeSequence;

    PrimeSequence::new()
        .take_while(|&i| i < 2_000_000)
        .fold(0, |acc, i| acc + i)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(142_913_828_922, super::p010());
    }
}
