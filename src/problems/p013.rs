//! Problem 13: Large sum

solve!(expecting_answer: 5_537_376_230, with: || {
    let sum = include_str!("../../resources/p013").lines()
        .filter_map(|line| line.chars().take(11).collect::<String>()
             .parse::<u64>().ok()
         ).sum::<u64>();

    (sum / 10_u64.pow((sum as f64).log10() as u32 - 9)) as u128
});
