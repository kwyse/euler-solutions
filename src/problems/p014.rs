//! Problem 14: Longest Collatz sequence

solve!(expecting_answer: 837_799, with: || {
    use std::collections::HashMap;

    struct CollatzSequence(Option<u64>);
    impl Iterator for CollatzSequence {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            let curr = self.0;
            self.0 = self.0.filter(|&n| n != 1)
                .map(|n| if n % 2 == 0 { n/2 } else { 3*n + 1 });

            curr
        }
    }

    let mut seq_lengths = HashMap::new();
    for i in 500_000..1_000_000 {
        let mut seq = CollatzSequence(Some(i));
        let mut len = 0;

        while let Some(next) = seq.next() {
            if let Some(found_len) = seq_lengths.get(&next) {
                seq_lengths.insert(i, len + found_len);
                break;
            } else {
                len += 1;
            }
        }

        seq_lengths.entry(i).or_insert(len);
    }

    seq_lengths.iter()
        .max_by_key(|&(_, v): &(&u64, &usize)| *v).map(|(&k, _)| k)
        .unwrap_or(0) as u128
});
