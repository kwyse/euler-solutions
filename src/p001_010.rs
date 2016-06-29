//! Tests for problems 1 through 10

#[test]
fn p001() {
    let actual = (1..1000)
        .filter(|i| (i % 3 == 0) || (i % 5 == 0))
        .fold(0, |acc, i| acc + i);

    assert_eq!(233168, actual);
}

#[test]
fn p002() {
    let actual = ::FibonacciSequence::new()
        .take_while(|&i| i < 4_000_000)
        .filter(|i| i % 2 == 0)
        .fold(0, |acc, i| acc + i);

    assert_eq!(4613732, actual);
}

#[test]
fn p003() {
    let target: u64 = 600851475143;
    let upper_bound = (target as f64).sqrt() as u64;

    let actual = ::PrimeSequence::new()
        .take_while(|&i| i < upper_bound)
        .filter(|i| target % i == 0)
        .max().unwrap();

    assert_eq!(6857, actual);
}

#[test]
fn p004() {
    let is_palindrome = |string: &str| {
        string.chars().zip(string.chars().rev()).all(|(c1, c2)| c1 == c2)
    };

    let mut palindromes: Vec<u32> = Vec::new();
    for i in (900..1000).rev() {
        for j in (900..1000).rev() {
            if is_palindrome(&(i * j).to_string()) {
                palindromes.push(i * j);
            }
        }
    }

    let actual = *palindromes.iter().max().unwrap();
    assert_eq!(906609, actual);
}
