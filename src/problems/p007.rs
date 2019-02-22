//! Problem 7: 10001st prime

solve!(expecting_answer: 104_743, with: || {
    struct PrimeGenerator(u32);
    impl Iterator for PrimeGenerator {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            let curr = self.0;
            self.0 = (self.0 + 2..).step_by(2).find(|&n| {
                (2..(n as f64).sqrt() as u32 + 1).all(|m| n % m != 0)
            }).unwrap();

            Some(curr)
        }
    }

    PrimeGenerator(3).nth(9999).unwrap() as u128
});
