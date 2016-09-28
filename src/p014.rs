#[test]
fn p014() {
    use ::collatz::CollatzSequence;

    let answer = (799_999..1_000_000u64)
        .map(|initial_val| (initial_val, CollatzSequence::new(initial_val).count()))
        .max_by_key(|&(_, sequence_length)| sequence_length)
        .unwrap()
        .0;

    assert_eq!(837799, answer);
}
