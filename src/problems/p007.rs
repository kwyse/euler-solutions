#[allow(dead_code)]
fn p007() -> u64 {
    use prime::PrimeSequence;

    PrimeSequence::new().nth(10_000).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(104_743, super::p007());
    }
}
