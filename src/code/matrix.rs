//! For dealing with matrices
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_from_str() {
        let matrix_str = "
            12 24 48
            10 20 30
            42 45 51
        ";

        let matrix = matrix_str.parse::<Matrix<u8>>().unwrap();
        assert_eq!(vec![12, 24, 48, 10, 20, 30, 42, 45, 51], matrix.items);
        assert_eq!(3, matrix.size);

        // TODO: Add malformed matrix test
    }

    #[test]
    fn test_matrix_adjacent_indexes() {
        use super::Direction::*;

        let matrix_str = "
            12 24 48
            10 20 30
            42 45 51
        ";

        let mat = matrix_str.parse::<Matrix<u8>>().unwrap();

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

        let mat = matrix_str.parse::<Matrix<u8>>().unwrap();

        assert_eq!(Some(vec![20, 51]), mat.items_at_indexes(vec![4, 8]));
        assert_eq!(None, mat.items_at_indexes(vec![1, 4, 7, 10]));
    }
}
