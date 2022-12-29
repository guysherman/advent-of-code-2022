use std::{fmt::Debug, str::FromStr, iter::{Rev, Skip}};

#[derive(Debug, Default, PartialEq)]
pub struct Matrix<T>
where
    T: FromStr + Debug + Copy,
{
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T: FromStr + Debug + Copy> Matrix<T> {
    pub fn from_string(input: &str) -> Matrix<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::<T>::new();

        for (i, line) in input.lines().enumerate() {
            if i == 0 {
                width = line.len();
            }

            if width != line.len() {
                panic!("Encountered a line that was not the same length as the first!");
            }

            height = i + 1;

            for j in 0..line.len() {
                let tree_height = line[j..j + 1].parse::<T>().unwrap();
                data.push(tree_height);
            }
        }

        Matrix {
            width,
            height,
            data,
        }
    }

    /// Takes an x, y (zero indexed!) and converts to an index in the vec
    pub fn index_from_point(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    pub fn col_iter(&self, column_index: usize) -> MatrixColumnIterator<T> {
        MatrixColumnIterator::for_column_of_matrix(self, column_index)
    }

    pub fn left_iter(&self, row_index: usize, column_index: usize) -> Rev<std::slice::Iter::<'_, T>> {
        let row_start = row_index * self.width;
        let datum = self.index_from_point(column_index, row_index);
        let slice = &self.data[row_start..datum];

        slice.iter().rev()
    }

    pub fn right_iter(&self, row_index: usize, column_index: usize) -> std::slice::Iter::<'_, T> {
        let datum = self.index_from_point(column_index, row_index);
        let row_end = (row_index + 1) * self.width;
        let slice = &self.data[(datum + 1)..row_end];

        slice.iter()
    }

    pub fn up_iter(&self, row_index: usize, column_index: usize) -> Skip<Rev<MatrixColumnIterator<T>>>{
        let to_skip = self.height - row_index;
        self.col_iter(column_index).rev().skip(to_skip)
    }

    pub fn down_iter(&self, row_index: usize, column_index: usize) -> Skip<MatrixColumnIterator<T>>{
        let to_skip = row_index + 1;
        self.col_iter(column_index).skip(to_skip)
    }

    pub fn vec_size(&self) -> usize {
        return self.width * self.height;
    }
}

pub struct MatrixColumnIterator<'a, T>
where
    T: FromStr + Debug + Copy,
{
    hm: &'a Matrix<T>,
    column_index: usize,
    front: usize,
    back: usize,
}

impl<'a, T: FromStr + Debug + Copy> MatrixColumnIterator<'a, T> {
    fn for_column_of_matrix(
        hm: &'a Matrix<T>,
        column_index: usize,
    ) -> MatrixColumnIterator<'a, T> {
        let back = hm.height;
        MatrixColumnIterator::<T> {
            hm,
            column_index,
            front: 0,
            back,
        }
    }
}

impl<'a, T: FromStr + Debug + Copy> Iterator for MatrixColumnIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.back == 0 {
            return None;
        }
        if self.front > (self.back - 1) {
            return None;
        }
        let vec_index = self.hm.index_from_point(self.column_index, self.front);
        let value = self.hm.data.get(vec_index);
        let result = match value {
            Some(val) => Some(val),
            None => None,
        };
        self.front += 1;
        result
    }
}

impl<'a, T: FromStr + Debug + Copy> DoubleEndedIterator for MatrixColumnIterator<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        if self.back == 0 {
            return None;
        }
        if self.front > (self.back - 1) {
            return None;
        }

        let vec_index = self.hm.index_from_point(self.column_index, self.back - 1);
        let value = self.hm.data.get(vec_index);
        let result = match value {
            Some(val) => Some(val),
            None => None,
        };
        self.back -= 1;
        result
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn given_test_input_matrix_from_string_has_correct_width() {
        let result = Matrix::<u8>::from_string(&INPUT);
        assert_eq!(result.width, 5);
    }

    #[test]
    fn given_test_input_matrix_from_string_has_correct_height() {
        let result = Matrix::<u8>::from_string(&INPUT);
        assert_eq!(result.height, 5);
    }

    #[test]
    fn given_test_input_matrix_from_string_has_correct_num_points() {
        let result = Matrix::<u8>::from_string(&INPUT);
        assert_eq!(result.data.len(), 25);
    }

    #[test]
    fn given_test_input_matrix_from_string_has_correct_value_at_point() {
        let result = Matrix::<u8>::from_string(&INPUT);
        assert_eq!(result.data[12], 3);
    }

    #[test]
    fn given_test_input_matrix_transforms_points_first_row() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let result = hm.index_from_point(2, 0);
        assert_eq!(result, 2);
    }

    #[test]
    fn given_test_input_matrix_transforms_points_last_row() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let result = hm.index_from_point(4, 4);
        assert_eq!(result, 24);
    }

    #[test]
    fn given_test_input_matrix_column_iterator_returns_correct_sequence() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let mut iter = hm.col_iter(2);
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn given_test_input_matrix_column_iterator_retruns_correct_back_sequence() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let mut iter = hm.col_iter(1);
        assert_eq!(iter.next_back(), Some(&5));
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next_back(), Some(&5));
        assert_eq!(iter.next_back(), Some(&5));
        assert_eq!(iter.next_back(), Some(&0));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn given_test_input_matrix_column_iterator_retruns_correct_rev_sequence() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let mut iter = hm.col_iter(1).rev();
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn given_input_left_iterator_for_2_2_returns_correct_sequence() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let mut iter = hm.left_iter(2, 2);

        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn given_input_right_iterator_for_2_2_returns_correct_sequence() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let mut iter = hm.right_iter(2, 2);

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test] 
    fn given_input_up_iterator_for_2_2_returns_correct_sequence() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let mut iter = hm.up_iter(2, 2);

        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

    }

    #[test] 
    fn given_input_down_iterator_for_2_2_returns_correct_sequence() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let mut iter = hm.down_iter(2, 2);

        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
}
