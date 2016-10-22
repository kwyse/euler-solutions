//! For dealing with triangle numbers
use std::ops::{Add, AddAssign};

/// Represents an sequence of triangle numbers
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_triangle_sequence() {
        let sequence = TriangleSequence::new().take(7).collect::<Vec<u64>>();

        assert_eq!(1, sequence[0]);
        assert_eq!(3, sequence[1]);
        assert_eq!(6, sequence[2]);
        assert_eq!(28, sequence[6]);
    }
}
