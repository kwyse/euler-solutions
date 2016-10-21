fn p017() -> u32 {
    use ut::string::ToWords;

    let mut words = "".to_string();
    for i in 1..1001 {
        words.push_str(&i.to_words().unwrap());
    }

    words.chars().filter(|&c| c != ' ').count() as u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p017() {
        assert_eq!(21124, super::p017());
    }
}
