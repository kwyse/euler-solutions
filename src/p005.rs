//! Problem 5: Smallest multiple
//!
//! 2520 is the smallest number that can be divided by each of the numbers from 1
//! to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the
//! numbers from 1 to 20?

pub fn lcm(ns: &[u64]) -> u64 {
    let mut lcm = 1;
    for window in ns.windows(2) {
        let (a, b) = (window[0], window[1]);
        let window_lcm = (a * b) / gcd(a, b);
        lcm = (lcm * window_lcm) / gcd(lcm, window_lcm);
    }

    lcm
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowest_common_multiple_is_calculated() {
        let ns = (1_u64..=10).collect::<Vec<_>>();
        assert_eq!(lcm(&ns), 2_520);
    }

    #[test]
    fn p005() {
        let ns = (1_u64..=20).collect::<Vec<_>>();
        assert_eq!(lcm(&ns), 232_792_560);
    }
}
