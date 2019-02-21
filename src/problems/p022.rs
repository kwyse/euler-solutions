//! Problem 22: Name scores

solve!(expecting_answer: 871_198_282, with: || {
    let mut names = include_str!("../../resources/p022")
        .trim_end_matches('\n').split(",").collect::<Vec<_>>();

    let calc_letter_scor = |letter| letter as u32 - 'A' as u32 + 1;

    names.sort_unstable();
    names.iter().enumerate().map(|(i, name)| {
            name.trim_matches('"').chars()
                .map(calc_letter_scor)
                .sum::<u32>() as u128 * (i as u128 + 1)
    }).sum::<u128>()
});

