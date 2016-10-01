//! Binomial theorem

/// How many subsets k can be made from set n
///
/// k greater than n is undefined.
///
/// If k is 0:
///   n!      n!
/// ------ = ---- = 1
/// (n-0)!    n!
///
/// If k > n / 2, we can call C(n, n - k) because coefficients are symmetrical
/// around the central binomial coefficient.
/// 
/// Otherwise, recurse because:
/// ( n )    n  ( n - 1 )
/// (   ) = --- (       )
/// ( k )    k  ( k - 1 )
///
pub fn n_choose_k(n: u64, k: u64) -> Option<u64> {
    if k > n {
        None
    } else if k == 0 {
        Some(1)
    } else if k > n / 2 {
        n_choose_k(n, n - k)
    } else if let Some(val) = n_choose_k(n - 1, k - 1) {
        Some(n * val / k)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_choose_k() {
        assert_eq!(None, n_choose_k(4, 8));
        assert_eq!(Some(1), n_choose_k(4, 0));
        assert_eq!(Some(4), n_choose_k(4, 1));
        assert_eq!(Some(4), n_choose_k(4, 3));

        assert_eq!(Some(2), n_choose_k(2, 1));
        assert_eq!(Some(6), n_choose_k(4, 2));
        assert_eq!(Some(20), n_choose_k(6, 3));
    }
}
