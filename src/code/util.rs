//! Misc. utility functions

/// Utility functions for strings or string slices
pub mod string {

    /// Doubles an unsigned integer represented as a string

    /// Iterates from the right side, doubling each individual character in
    /// turn and adding a carry when necessary. The string will grow by one
    /// if there is an outstanding carry.
    pub fn double_num(num: &str) -> String {
        let mut carry = false;
        let mut doubled_chars = num.chars()
            .rev()
            .map(|c| {
                let already_carry = carry;
                let mut x = c.to_digit(10).unwrap() * 2;

                if x > 9 {
                    x %= 10;
                    carry = true;
                } else {
                    carry = false;
                }

                if already_carry { x += 1 }
                (x as u8 + 0x30) as char
            })
            .collect::<Vec<char>>();

        if carry { doubled_chars.push('1'); }

        doubled_chars.iter()
            .rev()
            .map(|&c| c)
            .collect()
    }

    /// Sums the individual digits of an unsigned integer represented as a string
    pub fn sum_digits(num: &str) -> u32 {
        num.chars()
            .map(|c| c.to_digit(10).unwrap())
            .fold(0, |sum, x| sum + x)
    }
}

#[cfg(test)]
mod tests {
    use super::string::*;

    #[test]
    fn test_double_num() {
        assert_eq!("4", double_num("2"));
        assert_eq!("16", double_num("8"));
        assert_eq!("8268682", double_num("4134341"));
        assert_eq!("16537364", double_num("8268682"));
    }

    #[test]
    fn test_sum_digits() {
        assert_eq!(26, sum_digits("32768"));
    }
}
