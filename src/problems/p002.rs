#[allow(dead_code)]
fn p002() -> u64 {
    use fib::FibonacciSequence;

    FibonacciSequence::<u64>::new()
        .take_while(|&i| i < 4_000_000)
        .filter(|i| i % 2 == 0)
        .fold(0, |acc, i| acc + i) as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(4_613_732, super::p002());
    }
}
