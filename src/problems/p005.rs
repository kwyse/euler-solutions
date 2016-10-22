#[allow(dead_code)]
fn p005() -> u64 {
    let evenly_divisible = |i: &u64| (1..21).all(|j| *i % j == 0);
    (2520..).step_by(20).find(evenly_divisible).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(232_792_560, super::p005());
    }
}
