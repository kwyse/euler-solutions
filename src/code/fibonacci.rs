//! For dealing with a Fibonacci sequence
use std::ops::Add;

/// Represents an iterative Fibonacci sequence
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib_sequence() {
        let sequence: Vec<u64> = FibonacciSequence::new().take(6).collect();

        assert_eq!(&1, sequence.get(0).unwrap());
        assert_eq!(&1, sequence.get(1).unwrap());
        assert_eq!(&2, sequence.get(2).unwrap());
        assert_eq!(&3, sequence.get(3).unwrap());
        assert_eq!(&5, sequence.get(4).unwrap());
        assert_eq!(&8, sequence.get(5).unwrap());
    }
}
