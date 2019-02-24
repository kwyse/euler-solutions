//! Abstractions over Project Euler problems

#[macro_export]
macro_rules! solve {
    (expecting_answer: $expected:expr, with: $solution:expr) => {
        #[test]
        fn test() {
            let actual: u128 = $solution().into();
            assert_eq!(actual, $expected);
        }
    }
}

mod problems;
