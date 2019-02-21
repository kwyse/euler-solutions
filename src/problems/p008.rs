//! Problem 8: Largest product in a series

solve!(expecting_answer: 23_514_624_000, with: || {
    include_str!("../../resources/p008")
        .lines().collect::<Vec<_>>().concat()
        .chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>().windows(13)
        .map(|window| window.iter().map(|&n| n as u64).product::<u64>())
        .max().unwrap_or(0) as u128
});
