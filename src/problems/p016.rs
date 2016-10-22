#[allow(dead_code)]
fn p016() -> u64 {
    use ut::string;

    let mut num = "2".to_string();
    for _ in 1..1000 {
        num = string::double_num(&num);
    }

    string::sum_digits(&num) as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(1366, super::p016());
    }
}
