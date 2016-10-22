//! Tests for problems 1 through 10

#[test]
fn p001() {
    let actual = (1..1000)
        .filter(|i| (i % 3 == 0) || (i % 5 == 0))
        .fold(0, |acc, i| acc + i);

    assert_eq!(233_168, actual);
}

#[test]
fn p002() {
    let actual = ::FibonacciSequence::new()
        .take_while(|&i| i < 4_000_000)
        .filter(|i| i % 2 == 0)
        .fold(0, |acc, i| acc + i);

    assert_eq!(4_613_732, actual);
}

#[test]
fn p003() {
    use prime::PrimeSequence;

    let target: u64 = 600851475143;
    let upper_bound = (target as f64).sqrt() as u64;

    let actual = PrimeSequence::new()
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
    assert_eq!(906_609, actual);
}

#[test]
fn p005() {
    let evenly_divisible = |i: &u64| (1..21).all(|j| *i % j == 0);
    let actual = (2520..).step_by(20).find(evenly_divisible).unwrap();
    assert_eq!(232_792_560, actual);
}

#[test]
fn p006() {
    let sum = |acc, i| acc + i;
    let sum_of_squares = (1..101).map(|i| i * i).fold(0, &sum);
    let square_of_sum = (1..101).fold(0u64, &sum).pow(2);

    let actual = square_of_sum - sum_of_squares;
    assert_eq!(25_164_150, actual);
}

#[test]
fn p007() {
    use prime::PrimeSequence;

    let actual = PrimeSequence::new().nth(10_000).unwrap();
    assert_eq!(104_743, actual);
}

#[test]
fn p008() {
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

    let actual = *products.iter().max().unwrap();
    assert_eq!(23_514_624_000, actual);
}

#[test]
fn p009() {
    let is_triplet = |a, b, c| a*a + b*b == c*c;

    let mut triplets: Vec<(u64, u64, u64)> = Vec::new();
    let bound = 1000;
    for i in 1..bound {
        for j in i + 1..bound {
            for k in j + 1..bound {
                if is_triplet(i, j, k) {
                    triplets.push((i, j, k));
                }
            }
        }
    }

    let (a, b, c) = *triplets.iter()
        .find(|&&(a, b, c)| a + b + c == bound).unwrap();
    let actual = a * b * c;
    assert_eq!(31_875_000, actual);
}

#[test]
fn p010() {
    use prime::PrimeSequence;

    let actual = PrimeSequence::new()
        .take_while(|&i| i < 2_000_000)
        .fold(0, |acc, i| acc + i);
    assert_eq!(142_913_828_922, actual);
}
