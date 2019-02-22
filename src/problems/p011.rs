//! Problem 11: Largest product in a grid

solve!(expecting_answer: 70_600_674, with: || {
    let grid = include_str!("../../resources/p011")
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<_>>();

    fn product(grid: &[u32], indices: impl Iterator<Item = usize>) -> u32 {
        indices.map(|i| grid[i]).product::<u32>()
    }

    let right = |i| product(&grid, i..i + 4);
    let down = |i| product(&grid, (0..4).map(|j| i + j * 20));
    let down_right = |i| product(&grid, (0..4).map(|j| (i + j) + j * 20));
    let down_left = |i| product(&grid, (0..4).map(|j| (i - j) + j * 20));

    (0..=(20 * 16)).map(|i| match i % 20 {
        0...3 => right(i).max(down(i)).max(down_right(i)),
        16...19 => down(i).max(down_left(i)),
        _ => right(i).max(down(i)).max(down_right(i)).max(down_left(i))
    }).max().unwrap_or(0) as u128
});
