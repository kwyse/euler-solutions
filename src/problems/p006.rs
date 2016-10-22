#[allow(dead_code)]
fn p006() -> u64 {
    let sum = |acc, i| acc + i;
    let sum_of_squares = (1..101).map(|i| i * i).fold(0, &sum);
    let square_of_sum = (1..101).fold(0u64, &sum).pow(2);

    square_of_sum - sum_of_squares
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(25_164_150, super::p006());
    }
}
