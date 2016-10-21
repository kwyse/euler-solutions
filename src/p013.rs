#[test]
fn p013() {
    use ut::resource;
    let resource = resource::from_file("p013").unwrap();

    let answer: u64 = resource.lines()
        .map(|line| {
            String::from(line.chars().take(15).collect::<String>())
                .parse::<u64>()
                .unwrap()
        })
        .sum();

    assert_eq!("5537376230", &(answer.to_string())[0..10]);
}
