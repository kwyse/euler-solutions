#[allow(dead_code)]
fn p013() -> u64 {
    use ut::resource;
    let resource = resource::from_file("p013").unwrap();

    let answer: u64 = resource.lines()
        .map(|line| {
            String::from(line.chars().take(15).collect::<String>())
                .parse::<u64>()
                .unwrap()
        })
        .sum();

    answer.to_string()[0..10].parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(5_537_376_230, super::p013());
    }
}
