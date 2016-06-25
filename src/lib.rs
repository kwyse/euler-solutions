pub fn p001() -> u64 {
    (1..1000)
        .filter(|i| (i % 3 == 0) || (i % 5 == 0))
        .fold(0, |acc, i| acc + i)
}

pub fn p002() -> u64 {
    use std::u64;

    struct FibonacciSequence {
        a: u64,
        b: u64,
    }

    impl FibonacciSequence {
        fn new() -> Self {
            FibonacciSequence { a: 0, b: 1 }
        }
    }

    impl Iterator for FibonacciSequence {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            if self.b > u64::MAX {
                None
            } else {
                let tmp = self.b;
                self.b = self.a + self.b;
                self.a = tmp;
                Some(self.a)
            }
        }
    }

    FibonacciSequence::new()
        .take_while(|&i| i < 4_000_000)
        .filter(|i| i % 2 == 0)
        .fold(0, |acc, i| acc + i)
}
