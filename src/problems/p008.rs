#[allow(dead_code)]
fn p008() -> u64 {
    use ut::resource;

    let resource = resource::from_file("p008").unwrap();
    let buffer = resource.lines().collect::<String>();

    let mut products = Vec::new();

    for i in 0..1000 - 13 {
        products.push(buffer.chars()
                      .skip(i).take(13)
                      .map(|i| i.to_digit(10).unwrap() as u64)
                      .fold(1, |acc, i| acc * i)
        );
    }

    *products.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(23_514_624_000, super::p008());
    }
}
