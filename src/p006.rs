//! Problem 6: Sum square difference
//!
//! The sum of the squares of the first ten natural numbers is:
//!
//! 1² + 2² + ... + 10² = 385
//!
//! The square of the sum of the first ten natural numbers is:
//!
//! (1 + 2 + ... + 10)² = 55² = 3025
//!
//! Hence the difference between the sum of the squares of the first ten natural
//! numbers and the square of the sum is 3025 − 385 = 2640.
//!
//! Find the difference between the sum of the squares of the first one hundred
//! natural numbers and the square of the sum.

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|n| n.pow(2)).sum()
}

fn square_of_sum(n: u32) -> u32 {
    ((n * (n + 1)) / 2).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difference_between_sum_of_squares_and_square_of_sum_is_found() {
        assert_eq!(difference(10), 2_640);
    }

    #[test]
    fn sum_of_squares_is_found() {
        assert_eq!(sum_of_squares(10), 385);
    }

    #[test]
    fn square_of_sum_is_found() {
        assert_eq!(square_of_sum(10), 3_025);
    }

    #[test]
    fn p006() {
        assert_eq!(difference(100), 25_164_150);
    }
}
