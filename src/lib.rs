//! Abstractions over Project Euler problems

#![feature(step_by)]

pub use fib::FibonacciSequence;
pub use prime::PrimeSequence;
pub use mat::Matrix;
pub use tri::TriangleSequence;

mod p001_010;
mod p011_020;
mod p013;
mod resource;
mod util;

/// For dealing with a Fibonacci sequence
mod fib {
    use std::ops::Add;

    pub struct FibonacciSequence<T> {
        current: T,
        next: T,
    }

    impl<T: From<u8>> FibonacciSequence<T> {
        pub fn new() -> Self {
            FibonacciSequence {
                current: T::from(0),
                next: T::from(1)
            }
        }
    }

    impl<T> Iterator for FibonacciSequence<T>
        where T: Add<Output = T> + Copy {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            use std::mem;

            let new_next = self.current + self.next;
            self.current = mem::replace(&mut self.next, new_next);
            Some(self.current)
        }
    }
}

/// For dealing with prime numbers
mod prime {
    pub struct PrimeSequence {
        current: u64,
    }

    impl PrimeSequence {
        pub fn new() -> Self {
            PrimeSequence { current: 1 }
        }
    }

    impl Iterator for PrimeSequence {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            let not_prime = |i: &u64| !is_prime(i);
            self.current = (self.current + 1..).skip_while(not_prime).next().unwrap();
            Some(self.current)
        }
    }

    pub fn is_prime(num: &u64) -> bool {
        if *num == 2 || *num == 3 { true }
        else {
            let upper_bound = (*num as f64).sqrt() as u64 + 1;
            !(2..upper_bound).any(|i| num % i == 0)
        }
    }
}

/// For dealing with matrices
mod mat {
    use std::str::FromStr;
    use std::fmt::Debug;

    pub enum Direction {
        Right,
        DownRight,
        Down,
        DownLeft,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Matrix<T: FromStr> {
        pub items: Vec<T>,
        pub size: usize,
        current_index: usize,
    }

    impl<E: Debug, T: FromStr<Err = E> + Copy> Matrix<T> {
        /// Returns the specified number of adjacent indexes in a given direction
        /// from a given index, not including that index, or None a matrix
        /// boundary is hit.
        pub fn adjacent_indexes(&self, index: usize, num_items: usize, direction: &Direction) -> Option<Vec<usize>> {
            match direction {
                &Direction::Right => {
                    map_condition(
                        |i| index + i,
                        |&i| i / self.size == index / self.size,
                        num_items
                    )
                }

                &Direction::DownRight => {
                    map_condition(
                        |i| index + (self.size * i) + i,
                        |&i| (i % self.size) > (index % self.size) && self.items.get(i).is_some(),
                        num_items
                    )
                }

                &Direction::Down => {
                    map_condition(
                        |i| index + (self.size * i),
                        |&i| self.items.get(i).is_some(),
                        num_items
                    )
                }

                &Direction::DownLeft => {
                    map_condition(
                        |i| index + (self.size * i) - i,
                        |&i| (i % self.size) < (index % self.size) && self.items.get(i).is_some(),
                        num_items
                    )
                }
            }
        }

        pub fn items_at_indexes(&self, indexes: Vec<usize>) -> Option<Vec<T>> {
            let items = indexes.iter()
                .map(|&index| self.items.get(index).map(|item| *item))
                .take_while(Option::is_some)
                .map(Option::unwrap)
                .collect::<Vec<T>>();

            match items.len() == indexes.len() {
                true => Some(items),
                false => None,
            }
        }

    }

    impl<T: FromStr + Copy> Iterator for Matrix<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            let index = self.current_index;
            self.current_index += 1;
            self.items.get(index).map(|val| *val)
        }
    }

    impl<T: FromStr + Copy> AsRef<Matrix<T>> for Matrix<T> {
        fn as_ref(&self) -> &Matrix<T> {
            self
        }
    }

    impl<E: Debug, T: FromStr<Err = E>> FromStr for Matrix<T> {
        type Err = E;

        fn from_str(s: &str) -> Result<Matrix<T>, E> {
            let size_x = s.lines()
                .skip_while(|&line| line.is_empty())
                .next().unwrap_or_default()
                .split_whitespace()
                .count();

            let size_y = s.lines().filter(|&line| !line.is_empty()).count();

            let items = s.lines()
                .flat_map(str::split_whitespace)
                .map(str::parse::<T>)
                .take_while(|result| result.is_ok())
                .map(Result::unwrap)
                .collect::<Vec<T>>();

            match items.len() == size_x * size_y {
                true => Ok(Matrix {
                    items: items,
                    size: size_x,
                    current_index: 0,
                }),

                // TODO: This should return an Err
                false => Ok(Matrix {
                    items: items,
                    size: size_x,
                    current_index: 0,
                })
            }
        }
    }

    /// Returns the specified number of indexes according to the movement
    /// operation specified by `map` and the boundary condition specified by
    /// `condition`, or None if this condition is not satisfied for all items.
    fn map_condition<M, C>(map: M, condition: C, num_items: usize) -> Option<Vec<usize>>
        where M: FnMut(usize) -> usize, C: FnMut(&usize) -> bool {

        let indexes = (1..num_items + 1)
            .map(map)
            .take_while(condition)
            .collect::<Vec<usize>>();

        match indexes.len() == num_items {
            true => Some(indexes),
            false => None,
        }
    }
}

/// For dealing with triandgle numbers
mod tri{
    use std::ops::{Add, AddAssign};

    pub struct TriangleSequence<T> {
        current_value: T,
        count: T,
    }

    impl<T: From<u64>> TriangleSequence<T> {
        pub fn new() -> Self {
            TriangleSequence {
                current_value: T::from(1),
                count: T::from(1),
            }
        }
    }

    impl<T: Add<Output = T> + AddAssign + From<u64> + Copy> Iterator for TriangleSequence<T> {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            let old_value = self.current_value;
            self.current_value = old_value + self.count + T::from(1);
            self.count += T::from(1);

            Some(old_value)
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_fib_sequence() {
        let sequence: Vec<u64> = ::FibonacciSequence::new().take(6).collect();

        assert_eq!(&1, sequence.get(0).unwrap());
        assert_eq!(&1, sequence.get(1).unwrap());
        assert_eq!(&2, sequence.get(2).unwrap());
        assert_eq!(&3, sequence.get(3).unwrap());
        assert_eq!(&5, sequence.get(4).unwrap());
        assert_eq!(&8, sequence.get(5).unwrap());
    }

    #[test]
    fn test_is_prime() {
        use ::prime::is_prime;

        assert_eq!(is_prime(&2), true);
        assert_eq!(is_prime(&3), true);
        assert_eq!(is_prime(&4), false);
        assert_eq!(is_prime(&5), true);
        assert_eq!(is_prime(&8), false);
        assert_eq!(is_prime(&13), true);
        assert_eq!(is_prime(&27), false);
    }

    #[test]
    fn test_prime_sequence() {
        let sequence: Vec<u64> = ::PrimeSequence::new().take(10_001).collect();

        assert_eq!(&2, sequence.get(0).unwrap());
        assert_eq!(&3, sequence.get(1).unwrap());
        assert_eq!(&5, sequence.get(2).unwrap());
        assert_eq!(&104_743, sequence.get(10_000).unwrap());
    }

    #[test]
    fn test_matrix_from_str() {
        let matrix_str = "
            12 24 48
            10 20 30
            42 45 51
        ";

        let matrix = matrix_str.parse::<::Matrix<u8>>().unwrap();
        assert_eq!(vec![12, 24, 48, 10, 20, 30, 42, 45, 51], matrix.items);
        assert_eq!(3, matrix.size);

        // TODO: Add malformed matrix test
    }

    #[test]
    fn test_matrix_adjacent_indexes() {
        use super::mat::Direction::*;

        let matrix_str = "
            12 24 48
            10 20 30
            42 45 51
        ";

        let mat = matrix_str.parse::<::Matrix<u8>>().unwrap();

        assert_eq!(Some(vec![1, 2]), mat.adjacent_indexes(0, 2, &Right));
        assert_eq!(None, mat.adjacent_indexes(0, 3, &Right));
        assert_eq!(Some(vec![4, 8]), mat.adjacent_indexes(0, 2, &DownRight));
        assert_eq!(None, mat.adjacent_indexes(4, 2, &DownRight));
        assert_eq!(Some(vec![7]), mat.adjacent_indexes(4, 1, &Down));
        assert_eq!(None, mat.adjacent_indexes(4, 2, &Down));
        assert_eq!(Some(vec![4, 6]), mat.adjacent_indexes(2, 2, &DownLeft));
        assert_eq!(None, mat.adjacent_indexes(2, 3, &DownLeft));
    }

    #[test]
    fn test_matrix_items_at_indexes() {
        let matrix_str = "
            12 24 48
            10 20 30
            42 45 51
        ";

        let mat = matrix_str.parse::<::Matrix<u8>>().unwrap();

        assert_eq!(Some(vec![20, 51]), mat.items_at_indexes(vec![4, 8]));
        assert_eq!(None, mat.items_at_indexes(vec![1, 4, 7, 10]));
    }

    #[test]
    fn test_triangle_sequence() {
        let sequence = ::TriangleSequence::new().take(7).collect::<Vec<u64>>();

        assert_eq!(1, sequence[0]);
        assert_eq!(3, sequence[1]);
        assert_eq!(6, sequence[2]);
        assert_eq!(28, sequence[6]);
    }
}
