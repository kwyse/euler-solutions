//! Abstractions over Project Euler problems

#![feature(step_by)]

pub use fib::FibonacciSequence;
pub use prime::PrimeSequence;
pub use mat::Matrix;

mod p001_010;
mod p011_020;
mod resource;

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
        matrix: Vec<T>,
        size_x: usize,
        current: usize,
        // current_x: usize,
        // current_y: usize,
    }

    impl<E: Debug, T: FromStr<Err = E> + Copy> Matrix<T> {
        pub fn new(matrix_str: &str) -> Self {
            let matrix = matrix_str.parse::<Self>().unwrap();
            matrix
        }

        pub fn get_n_adjacent_indexes(&self, index: usize, n: usize, direction: &Direction) -> Option<Vec<usize>> {
            let mut vec = Vec::with_capacity(n);

            match direction {
                &Direction::Right => {
                    let starting_row = index / self.size_x;
                    for i in 1..n + 1 {
                        let current_index = index + i;
                        let current_row = current_index / self.size_x;

                        if starting_row == current_row {
                            vec.push(current_index);
                        } else {
                            break;
                        }
                    }

                    match vec.len() == n {
                        true => Some(vec),
                        false => None,
                    }
                }

                &Direction::DownRight => {
                    let old_column = index % self.size_x;
                    for i in 1..n + 1 {
                        let current_index = index + (self.size_x * i) + i;

                        let current_column = current_index % self.size_x;
                        if current_column > old_column && self.matrix.get(current_index).is_some() {
                            vec.push(current_index);
                        } else {
                            break;
                        }
                    }

                    match vec.len() == n {
                        true => Some(vec),
                        false => None,
                    }
                }

                &Direction::Down => {
                    for i in 1..n + 1 {
                        let current_index = index + (self.size_x * i);
                        if self.matrix.get(current_index).is_some() {
                            vec.push(current_index);
                        } else {
                            break;
                        }
                    }

                    match vec.len() == n {
                        true => Some(vec),
                        false => None,
                    }
                }

                &Direction::DownLeft => {
                    let old_column = index % self.size_x;
                    for i in 1..n + 1 {
                        let current_index = index + (self.size_x * i) - i;

                        let current_column = current_index % self.size_x;
                        if current_column < old_column && self.matrix.get(current_index).is_some() {
                            vec.push(current_index);
                        } else {
                            break;
                        }
                    }

                    match vec.len() == n {
                        true => Some(vec),
                        false => None,
                    }
                }
            }
        }

        pub fn get_elements(&self, indexes: Vec<usize>) -> Option<Vec<T>> {
            let mut elements: Vec<T> = Vec::new();

            for index in &indexes {
                if let Some(element) = self.matrix.get(*index) {
                    elements.push(*element);
                }
            }

            match elements.len() == indexes.len() {
                true => Some(elements),
                false => None,
            }
        }
    }

    impl<T: FromStr + Copy> Iterator for Matrix<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            let index = self.current;
            self.current += 1;
            self.matrix.get(index).map(|val| *val)
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
                // .skip_while(str::is_empty)
                .next().unwrap_or_default()
                .split_whitespace()
                .count();

            let vec = s.lines()
                .flat_map(str::split_whitespace)
                .map(str::parse::<T>)
                .map(Result::unwrap)
                .collect();

            Ok(Matrix {
                matrix: vec,
                size_x: size_x,
                current: 0,
            })
            // let flattened_matrix = s.lines()
            //     .flat_map(str::split_whitespace)
            //     .collect::<Vec<&str>>();

            // let parsed_results = flattened_matrix.iter()
            //     .map(|val| val.parse::<T>())
            //     .collect::<Vec<Result<_, _>>>();

            // if parsed_results.iter().any(|&item| item.is_err()) {
            //     let r: Result<Matrix<T>, E> = *parsed_results.iter().find(|&&item| item.is_err()).unwrap();
            //     r
            // } else {
            //     let vec = parsed_results.iter().map(|val| val.unwrap()).collect();
            //     Ok(Matrix {
            //         matrix: vec,
            //         size_x: size_x,
            //     })
            // }

            // let mut all_parsed = true;

            // for val in s.lines().flat_map() {
            //     try!(j
            //parsed_results.iter().find(move |item| item.is_err()).unwrap_or_else(|_|
            // parsed_results.find(|item| item == Err(_)).unwrap_or_else(|item| {

            // if flattened_matrix.len() != parsed_results.len() {
            //     parsed_results.find(|item| item == Err(_)).unwrap().err().unwrap()
            // } else {



            // let matrix = s.lines().flat_map(|val| {
            //     val.parse::<T>().unwrap_or_else(|err| 

            // let matrix = s.lines().map(|line| {
            //     line.split_whitespace().map(|val| {
            //         val.parse::<T>().unwrap(|err| {
            //             all_parsed = false;
            //             T::default()
            //         })
            //     }).collect()
            // }).collect();

            // Ok(Matrix { matrix: matrix })
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
}
