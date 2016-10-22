#[allow(dead_code)]
fn p001() -> u64 {
    (1..1000)
        .filter(|i| (i % 3 == 0) || (i % 5 == 0))
        .fold(0, |acc, i| acc + i)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(233_168, super::p001());
    }
}
