//! Problem 5: Smallest multiple
//!
//! 2520 is the smallest number that can be divided by each of the numbers from 1
//! to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the
//! numbers from 1 to 20?

pub fn smallest_multiple(start: u32, end: u32) -> Option<u32> {
    (end..)
        .step_by(end as usize)
        .find(|n| (start..end).all(|m| n % m == 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smallest_multiples_are_found() {
        assert_eq!(smallest_multiple(1, 10), Some(2_520));
    }

    #[test]
    fn p005() {
        assert_eq!(smallest_multiple(1, 20), Some(232_792_560));
    }
}
