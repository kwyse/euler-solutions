#[allow(dead_code)]
fn p015() -> u64 {
    use bi;

    bi::n_choose_k(40, 20).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p015() {
        assert_eq!(137846528820, super::p015());
    }
}
