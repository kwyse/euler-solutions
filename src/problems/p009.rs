#[allow(dead_code)]
fn p009() -> u64 {
    let is_triplet = |a, b, c| a * a + b * b == c * c;

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

    let (a, b, c) = *triplets
        .iter()
        .find(|&&(a, b, c)| a + b + c == bound)
        .unwrap();

    a * b * c
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(31_875_000, super::p009());
    }
}
