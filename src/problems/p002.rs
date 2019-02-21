//! Problem 2: Even Fibonacci numbers

solve!(expecting_answer: 4_613_732, with: || {
    struct Fib(u32, u32);

    impl Iterator for Fib {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            let curr = self.0;
            self.0 = self.1;
            self.1 += curr;
            Some(curr)
        }
    }

    Fib(1, 2)
        .take_while(|&n| n < 4_000_000)
        .filter(|n| n % 2 == 0)
        .sum::<u32>() as u128
});
