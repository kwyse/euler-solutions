#[allow(dead_code)]
fn p017() -> u64 {
    use ut::string::ToWords;

    let mut words = "".to_string();
    for i in 1..1001 {
        words.push_str(&i.to_words().unwrap());
    }

    words.chars().filter(|&c| c != ' ').count() as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(21_124, super::p017());
    }
}
