fn p016() -> u32 {
    use ut::string;

    let mut num = "2".to_string();
    for _ in 1..1000 {
        num = string::double_num(&num);
    }

    string::sum_digits(&num)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p016() {
        assert_eq!(1366, super::p016());
    }
}
