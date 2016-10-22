#[allow(dead_code)]
fn p014() -> u64 {
    use collatz::CollatzSequence;

    (799_999..1_000_000u64)
        .map(|initial_val| (initial_val, CollatzSequence::new(initial_val).count()))
        .max_by_key(|&(_, sequence_length)| sequence_length)
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(837_799, super::p014());
    }
}
