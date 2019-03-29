//! Abstractions over Project Euler problems

#[macro_export]
macro_rules! solve {
    (expecting_answer: $expected:expr, with: $solution:expr) => {
        #[test]
        fn test() {
            let actual = $solution();
            assert_eq!(actual, $expected);
        }
    };
}

pub mod problems;
