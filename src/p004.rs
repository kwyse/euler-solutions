//! Problem 4: Largest palindrome product
//!
//! A palindromic number reads the same both ways. The largest palindrome made
//! from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

/// Finds the largest palindromic product within a range of numbers
///
/// For each number yielded in succession from the highest number to the lowest,
/// check if this number if divisible by _11_. All decimal palindromic numbers
/// with an even number of digits are divisble by _11_.
///
/// For example, for a six digit palindromic number _P_, with letters _x_, _y_,
/// and _z_ representing each of its six digits:
///
/// _P_
/// = 100000x + 10000y + 1000z + 100z + 10y + x
/// = 100001x + 10010y + 1100z
/// = 11(9091x + 910y + 100z)
///
/// Or for a four digit palindromic number:
///
/// _P_
/// = 1000x + 100y + 10y + x
/// = 1001x + 110y
/// = 11(91x + 10y)
///
/// If the yeilded number is not divisible by _11_, the other factor of the
/// potentially palindromic number is, so increment by _11_ in each inner loop
/// iteration. Within the inner loop, first check if the product is smaller than
/// the current largest found palindrome, and break early if so since subsequent
/// palindromes will always be smaller. If not, check if the product is a
/// palindrome by comparing it to its reversed value.
pub fn largest_palindrome(lo: u32, hi: u32) -> u32 {
    let mut largest = 0;

    for x in (lo..=hi).rev() {
        let mut y = hi;
        let mut dy = 1;
        if x % 11 != 0 {
            y = hi / 11 * 11;
            dy = 11;
        }

        while y >= x {
            let product = x * y;
            if product <= largest {
                break;
            } else if product == reverse(product) {
                largest = product;
            }

            y -= dy;
        }
    }

    largest
}

fn reverse(mut n: u32) -> u32 {
    let mut reversed = 0;
    while n > 0 {
        reversed = (reversed * 10) + (n % 10);
        n /= 10;
    }

    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_palindrome_is_found() {
        assert_eq!(largest_palindrome(10, 99), 9_009);
    }

    #[test]
    fn p004() {
        assert_eq!(largest_palindrome(100, 999), 906_609);
    }
}
