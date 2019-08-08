#[allow(dead_code)]
fn p004() -> u64 {
    let is_palindrome = |string: &str| {
        string
            .chars()
            .zip(string.chars().rev())
            .all(|(c1, c2)| c1 == c2)
    };

    let mut palindromes: Vec<u32> = Vec::new();
    for i in (900..1000).rev() {
        for j in (900..1000).rev() {
            if is_palindrome(&(i * j).to_string()) {
                palindromes.push(i * j);
            }
        }
    }

    *palindromes.iter().max().unwrap() as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(906_609, super::p004());
    }
}
