//! Problem 16: Power digit sum

solve!(expecting_answer: 1366, with: || {
    let double = |s: &str| {
        let mut carry = false;
        s.chars().rev().chain(['0'].into_iter().cloned()).filter_map(|c| {
            c.to_digit(10).map(|d| {
                let doubled = d * 2 + (if carry { 1 } else { 0 });
                if doubled / 10 > 0 {
                    carry = true;
                    doubled % 10
                } else {
                    carry = false;
                    doubled
                }
            }).map(|d| (d as u8 + '0' as u8) as char)
        })
        .collect::<String>().trim_matches('0').chars().rev().collect()
    };

    let mut n = 1.to_string();
    for _ in 0..1000 { n = double(&n) }
    n.chars().filter_map(|c| c.to_digit(10)).sum::<u32>() as u128

});
