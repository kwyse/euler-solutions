//! Problem 20: Factorial digit sum

solve!(expecting_answer: 648, with: || {
    use std::fmt::{ self, Display, Formatter };

    const BASE: u64 = 1_000_000_000;

    struct BigInt(Vec<u64>);
    impl BigInt {
        fn mult(&mut self, other: u64) {
            let mut carry = 0;
            for i in 0..self.0.len() {
                let tmp = other * self.0[i] + carry;
                self.0[i] = tmp % BASE;
                carry = tmp / BASE;
            }

            if carry > 0 { self.0.push(carry) }
        }
    }

    impl Display for BigInt {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let print_ordered = self.0.iter().rev().collect::<Vec<_>>();
            write!(f, "{}", print_ordered.get(0).map(ToString::to_string)
                    .unwrap_or("".to_string()))?;
            write!(f, "{}", print_ordered[1..].iter()
                    .map(|s| format!("{:02}", s)).collect::<String>())
        }
    }

    let mut factorial = BigInt(vec![1]);
    for i in 1..=100 {
        factorial.mult(i);
    }

    factorial.to_string().chars().filter_map(|c| c.to_digit(10)).sum::<u32>()
});
