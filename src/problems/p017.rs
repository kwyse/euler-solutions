//! Problem 17: Number letter counts

solve!(expecting_answer: 21_124, with: || {
    let words = |n: u32| {
        let thousands = if n >= 1000 { "one thousand" } else { "" };
        let singles = |matcher| match matcher {
            1 => "one", 2 => "two", 3 => "three", 4 => "four", 5 => "five",
            6 => "six", 7 => "seven", 8 => "eight", 9 => "nine",
            _ => "",
        };

        let ones = singles(n % 10);
        let mut hundreds = singles((n % 1000) / 100).to_string();
        if !hundreds.is_empty() {
            hundreds.push_str("hundred");
            if n % 100 != 0 { hundreds.push_str("and") }
        }

        let teens = match n % 100 {
            11 => "eleven", 12 => "twelve", 13 => "thirteen", 14 => "fourteen",
            15 => "fifteen", 16 => "sixteen", 17 => "seventeen",
            18 => "eighteen", 19 => "nineteen",
            _ => "",
        };
        
        let tens = match (n % 100) / 10 {
            1 => "ten", 2 => "twenty", 3 => "thirty", 4 => "forty",
            5 => "fifty", 6 => "sixty", 7 => "seventy", 8 => "eighty",
            9 => "ninety",
            _ => "",
        };
        
        if n % 100 > 10 && n % 100 < 20 { 
            vec![ thousands, &hundreds, teens ]
        } else { 
            vec![ thousands, &hundreds, tens, ones ]
        }
            .join(" ").replace(" ", "")
    };

    (1..=1000).map(|n| dbg!(words(n)).len()).sum::<usize>() as u128
});
