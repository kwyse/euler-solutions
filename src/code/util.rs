//! Misc. utility functions

/// Utility functions for resources stored in files
pub mod resource {
    use std::io;
    use std::path::Path;

    /// Reads a problem resource from disk
    pub fn from_file<P: AsRef<Path>>(resource: P) -> Result<String, io::Error> {
        use std::path::PathBuf;

        let mut path = PathBuf::from("resources");
        path.push(resource);

        string_from_file(path)
    }

    fn string_from_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
        use std::fs::File;
        use std::io::Read;

        let mut file = try!(File::open(path));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));
        Ok(contents)
    }
}

/// Utility functions for strings or string slices
pub mod string {

    /// Doubles an unsigned integer represented as a string
    ///
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

    /// Converts a number to its words (in British Enligh) equivelent
    pub trait ToWords {
        fn to_words(&self) -> Option<String>;
    }

    impl ToWords for u32 {
        fn to_words(&self) -> Option<String> {
            let num_as_string = self.to_string();
            match num_as_string.len() {
                1 => Some(ones(num_as_string.chars().next().unwrap())),
                2 => Some(tens(&num_as_string)),
                3 => Some(hundreds(&num_as_string)),
                4 => Some(thousands(&num_as_string)),
                _ => None
            }
        }
    }

    /// Converts a one digit number into words
    fn ones(num: char) -> String {
        let as_str = match num {
            '0' => "zero", '1' => "one", '2' => "two", '3' => "three", '4' => "four",
            '5' => "five", '6' => "six", '7' => "seven", '8' => "eight", '9' => "nine",
            _ => unreachable!()
        };

        as_str.to_string()
    }

    /// Converts a two digit number into words
    fn tens(num: &str) -> String {
        let mut chars = num.chars();
        let as_str = match (chars.next().unwrap(), chars.next().unwrap()) {
            ('0', one) => ones(one),
            ('1', '0') => "ten".to_string(), ('1', '1') => "eleven".to_string(),
            ('1', '2') => "twelve".to_string(), ('1', '3') => "thirteen".to_string(),
            ('1', '5') => "fifteen".to_string(), ('1', '8') => "eighteen".to_string(),
            ('2', '0') => "twenty".to_string(), ('3', '0') => "thirty".to_string(),
            ('4', '0') => "forty".to_string(), ('5', '0') => "fifty".to_string(),
            ('8', '0') => "eighty".to_string(),
            ('1', one) => format!("{}{}", ones(one), "teen"),
            ('2', one) => format!("{} {}", "twenty", ones(one)),
            ('3', one) => format!("{} {}", "thirty", ones(one)),
            ('4', one) => format!("{} {}", "forty", ones(one)),
            ('5', one) => format!("{} {}", "fifty", ones(one)),
            ('8', one) => format!("{} {}", "eighty", ones(one)),
            (ten, '0') => format!("{}ty", ones(ten)),
            (ten, one) => format!("{}ty {}", ones(ten), ones(one))
        };

        as_str.to_string()
    }

    /// Converts a three digit number into words
    fn hundreds(num: &str) -> String {
        let hundred = num.chars().next().unwrap();
        let ten = &num[1..3];

        if ten == "00" {
            format!("{} hundred", ones(hundred))
        } else {
            format!("{} hundred and {}", ones(hundred), tens(ten))
        }
    }

    /// Converts a four digit number into words
    fn thousands(num: &str) -> String {
        let mut chars = num.chars();
        let thousand = chars.next().unwrap();
        let hundred = chars.next().unwrap();
        let ten = &num[2..4];

        if &num[1..4] == "000" {
            format!("{} thousand", ones(thousand))
        } else if &num[2..4] == "00" {
            format!("{} thousand {} hundred", ones(hundred), tens(ten))
        } else {
            format!("{} thousand {} hundred and {}", ones(thousand), ones(hundred), tens(ten))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::string::*;

    // TODO: Add resource module tests

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

    #[test]
    fn test_to_words() {
        assert_eq!(None, 700819.to_words()); // too big
        assert_eq!(Some("forty two".to_string()), 42.to_words());
        assert_eq!(Some("four hundred and fifty one".to_string()), 451.to_words());
        assert_eq!(Some("nine thousand".to_string()), 9000.to_words());
        assert_eq!(Some("eight hundred".to_string()), 800.to_words());
        assert_eq!(Some("eight hundred and one".to_string()), 801.to_words());
    }
}
