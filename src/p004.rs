//! Problem 4: Largest palindrome product
//!
//! A palindromic number reads the same both ways. The largest palindrome made
//! from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

use std::ops::Range;

pub fn largest_palindrome(range: Range<u32>) -> u32 {
    range
        .clone()
        .flat_map(|n| range.clone().map(move |m| n * m))
        .filter(|n| {
            let s = n.to_string();
            s.chars().rev().collect::<String>() == s
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_palindrome_is_found() {
        assert_eq!(largest_palindrome(80..100), 9009);
    }

    #[test]
    fn p004() {
        assert_eq!(largest_palindrome(800..1_000), 906_609);
    }
}
