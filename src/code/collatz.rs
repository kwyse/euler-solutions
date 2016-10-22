//! Collatz conjecture
use std::ops::{Add, Div, Mul, Rem};

/// Represents the sequence of the Collatz conjecture
pub struct CollatzSequence<T> {
    current: T,
}

impl<T> CollatzSequence<T> {
    pub fn new(first: T) -> Self {
        CollatzSequence {
            current: first
        }
    }
}

impl<T> Iterator for CollatzSequence<T>
    where T: From<u8> + Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + PartialEq {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.current == T::from(1) {
            None
        } else {
            if self.current % T::from(2) == T::from(0) {
                self.current = self.current / T::from(2);
            } else {
                self.current = self.current * T::from(3) + T::from(1);
            }

            Some(self.current)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_collatz_sequence() {
        let sequence = CollatzSequence::new(13);
        assert_eq!(9, sequence.count());
    }
}
