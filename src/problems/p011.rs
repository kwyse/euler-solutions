//! Problem 11: Largest product in a grid

solve!(expecting_answer: 70_600_674, with: || {
    let grid = include_str!("../../resources/p011")
        .split_whitespace()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let calc_product = |movement_op: &Fn(usize) -> usize| -> u32 {
        (0..4).map(movement_op).map(|i| grid[i]).product::<u32>()
    };

    let right = |i| calc_product(&|j| i + j);
    let down = |i| calc_product(&|j| i + j * 20);
    let down_right = |i| calc_product(&|j| (i + j) + j * 20);
    let down_left = |i| calc_product(&|j| (i - j) + j * 20);

    (0..=(20 * 16)).map(|i| match i % 20 {
        0...3 => right(i).max(down(i)).max(down_right(i)),
        16...19 => down(i).max(down_left(i)),
        _ => right(i).max(down(i)).max(down_right(i)).max(down_left(i))
    }).max().unwrap_or(0) as u128
});
