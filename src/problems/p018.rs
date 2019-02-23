// Problem 18: Maximum path sum I

solve!(expecting_answer: 1074, with: || {
    fn deserialize(input: &str) -> Vec<Vec<u32>> {
        input.lines().map(str_line_to_u32s).collect()
    }

    fn str_line_to_u32s(line: &str) -> Vec<u32> {
        line.split_whitespace().filter_map(parse_u32).collect()
    }

    fn parse_u32(num: &str) -> Option<u32> {
        str::parse(num).ok()
    }

    let mut prev = vec![0, 0];

    let triangle = deserialize(include_str!("../../resources/p018"));
    for line in triangle {
        let mut curr = vec![0];

        for (i, n) in line.iter().enumerate() {
            curr.push((n + prev[i]).max(n + prev[i + 1]))
        }

        curr.push(0);
        prev = curr
    }

    prev.iter().max().map(Clone::clone).unwrap_or(0) as u128
});
