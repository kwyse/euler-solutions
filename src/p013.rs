#[test]
fn p013() {
    let resource = ::util::resource_from_file("p013").unwrap();

    let answer: u64 = resource.lines()
        .map(|line| {
            String::from(
                line.chars()
                    .take(15)
                    .collect::<Vec<char>>())
                .parse::<u64>()
                .unwrap()
        })
        .sum();

    assert_eq!("5537376230", &(answer.to_string())[0..10]);
}
