//! Problem 4: Largest palindrome product
//!
//! A palindromic number reads the same both ways. The largest palindrome made
//! from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

pub fn largest_palindrome(lo: u32, hi: u32) -> u32 {
    let mut largest = 0;

    for n in (lo..hi).rev() {
        let mut m = hi;

        while m >= n {
            let product = n * m;
            if product < largest {
                break;
            } else if product == reverse(product) {
                largest = product;
            }

            m -= 1;
        }
    }

    largest
}

fn reverse(mut n: u32) -> u32 {
    let mut reversed = 0;
    while n > 0 {
        reversed = (10 * reversed) + (n % 10);
        n /= 10;
    }

    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_palindrome_is_found() {
        assert_eq!(largest_palindrome(1, 100), 9009);
    }

    #[test]
    fn p004() {
        assert_eq!(largest_palindrome(1, 1_000), 906_609);
    }
}
