use std::{fs::read_to_string, str::FromStr, fmt::Debug};


#[derive(Debug, Default, PartialEq)]
struct Matrix<T> where T: FromStr + Debug + Copy {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: FromStr + Debug + Copy> Matrix<T> {
    fn from_string(input: &str) -> Matrix<T> where <T as FromStr>::Err: Debug {
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

            height = i+1;

            for j in 0..line.len() {
                let tree_height = line[j..j+1].parse::<T>().unwrap();
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
    fn index_from_point(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn col_iter(&self, column_index: usize) -> MatrixColumnIterator<T> {
        MatrixColumnIterator::for_column_of_heightmap(self, column_index)
    }

    fn vec_size(&self) -> usize {
        return self.width * self.height;
    }
}

struct MatrixColumnIterator<'a, T> where T: FromStr + Debug + Copy {
    hm: &'a Matrix<T>,
    column_index: usize,
    front: usize,
    back: usize,
}

impl<'a, T: FromStr + Debug + Copy> MatrixColumnIterator<'a, T> {
    fn for_column_of_heightmap(hm: &'a Matrix<T>, column_index: usize) -> MatrixColumnIterator<'a, T> {
        let back = hm.height;
        MatrixColumnIterator::<T> { hm, column_index, front: 0, back }
    }
}

impl<'a, T: FromStr + Debug + Copy> Iterator for MatrixColumnIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.back == 0 {
            return None
        }
        if self.front > (self.back - 1)  {
            return None
        }
        let vec_index = self.hm.index_from_point(self.column_index, self.front);
        let value = self.hm.data.get(vec_index);
        let result = match value {
            Some(val) => Some(*val),
            None => None
        };
        self.front += 1;
        result
    }
}

impl<'a, T: FromStr + Debug + Copy> DoubleEndedIterator for MatrixColumnIterator<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        if self.back == 0 {
            return None
        }
        if self.front > (self.back - 1) {
            return None
        }

        let vec_index = self.hm.index_from_point(self.column_index, self.back - 1);
        let value = self.hm.data.get(vec_index);
        let result = match value {
            Some(val) => Some(*val),
            None => None
        };
        self.back -= 1;
        result
    }
}


fn main() {
    let input = read_to_string("input.txt").unwrap();
    let result = count_visible_trees(&input);
    println!("{}", result);
}

fn count_visible_trees(input: &str) -> usize {
    // Parse the text into a HeightMap (which is a Vec<u8> plus height and width)
    let hm = Matrix::<u8>::from_string(&input);
    // read through each row forwards and backwards, and then each col forwards and backwards,
    let mut vm = vec![false; hm.vec_size()];
    compute_visibility_left(&hm, &mut vm);
    compute_visibility_right(&hm, &mut vm);
    compute_visibility_down(&hm, &mut vm);
    compute_visibility_up(&hm, &mut vm);
    

    // computing 4 visibility maps (one for each direction). At each point if the current tree is
    // taller than the tallest tree so far in that row/col then it is visible, and it is now the
    // tallest tree so far. You could actually just do one visibility map... visibility is the
    // existing val OR the current val
    // HeightMap should have some index->point and point->index conversions
    vm.iter().filter(|x| **x).count()
}

fn compute_visibility_left(hm: &Matrix<u8>, vm: &mut Vec<bool>) {
    for r in 0..hm.height {
        let row = &hm.data[(r * hm.width)..((r+1) * hm.width)];
        let mut max = 0u8;
        for (c, col) in row.iter().enumerate() {
            let index = hm.index_from_point(c, r);
            let mut visibility = vm[index];
            visibility = compute_individual_visibility(&mut max, c, *col, visibility);
            vm.splice(index..index+1, [visibility]);
        }
    }
}

fn compute_individual_visibility(max: &mut u8, c: usize, height: u8, visibility: bool) -> bool {
    let mut result_visibility = visibility;
    if c == 0 {
        result_visibility |= true;
        *max = height;
    } else {
        if height > *max {
            result_visibility |= true;
            *max = height;
        }
    }
    result_visibility
}

fn compute_visibility_right(hm: &Matrix<u8>, vm: &mut Vec<bool>) {
    for r in 0..hm.height {
        let row = &hm.data[(r * hm.width)..((r+1) * hm.width)];
        let mut max = 0u8;
        for (c, col) in row.iter().rev().enumerate() {
            let index = hm.index_from_point(hm.width - c - 1, r);
            let mut visibility = vm[index];
            visibility = compute_individual_visibility(&mut max, c, *col, visibility);
            vm.splice(index..index+1, [visibility]);
        }
    }
}

fn compute_visibility_down(hm: &Matrix<u8>, vm: &mut Vec<bool>) {
    for c in 0..hm.width {
        let col = hm.col_iter(c);
        let mut max = 0u8;
        for (r, row) in col.enumerate() {
            let index = hm.index_from_point(c, r);
            let mut visibility = vm[index];
            visibility = compute_individual_visibility(&mut max, r, row, visibility);
            vm.splice(index..index+1, [visibility]);

        }
    }
}

fn compute_visibility_up(hm: &Matrix<u8>, vm: &mut Vec<bool>) {
    for c in 0..hm.width {
        let col = hm.col_iter(c);
        let mut max = 0u8;
        for (r, row) in col.rev().enumerate() {
            let index = hm.index_from_point(c, hm.height - r - 1);
            let mut visibility = vm[index];
            visibility = compute_individual_visibility(&mut max, r, row, visibility);
            vm.splice(index..index+1, [visibility]);
        }
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
    fn given_test_input_count_visible_trees_returns_21() {
        let result = count_visible_trees(&INPUT);
        assert_eq!(result, 21);
    }

    #[test]
    fn given_test_input_heightmap_from_string_has_correct_width() {
        let result = Matrix::<u8>::from_string(&INPUT);
        assert_eq!(result.width, 5);
    }

    #[test]
    fn given_test_input_heightmap_from_string_has_correct_height() {
        let result = Matrix::<u8>::from_string(&INPUT);
        assert_eq!(result.height, 5);
    }

    #[test]
    fn given_test_input_heightmap_from_string_has_correct_num_points() {
        let result = Matrix::<u8>::from_string(&INPUT);
        assert_eq!(result.data.len(), 25);
    }

    #[test]
    fn given_test_input_heightmap_from_string_has_correct_value_at_point() {
        let result = Matrix::<u8>::from_string(&INPUT);
        assert_eq!(result.data[12], 3);
    }

    #[test]
    fn given_test_input_heightmap_transforms_points_first_row() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let result = hm.index_from_point(2, 0);
        assert_eq!(result, 2);
    }

    #[test]
    fn given_test_input_heightmap_transforms_points_last_row() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let result = hm.index_from_point(4, 4);
        assert_eq!(result, 24);
    }

    #[test]
    fn given_test_input_heightmap_column_iterator_returns_correct_sequence() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let mut iter = hm.col_iter(2);
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn given_test_input_heightmap_column_iterator_retruns_correct_back_sequence() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let mut iter = hm.col_iter(1);
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next_back(), Some(0));
        assert_eq!(iter.next_back(), None);

    }

    #[test]
    fn given_test_input_heightmap_column_iterator_retruns_correct_rev_sequence() {
        let hm = Matrix::<u8>::from_string(&INPUT);
        let mut iter = hm.col_iter(1).rev();
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);

    }

    #[test]
    fn given_row_compute_visibility_left_returns_correct_map() {
        let hm = Matrix::<u8>::from_string("12321");
        let mut vm = vec![false; hm.vec_size()];
        compute_visibility_left(&hm, &mut vm);
        assert_eq!(vm, vec![true, true, true, false, false]);
    }

    #[test]
    fn given_rows_compute_visibility_left_returns_correct_map() {
        let hm = Matrix::<u8>::from_string("12321\n32111");
        let mut vm = vec![false; hm.vec_size()];
        compute_visibility_left(&hm, &mut vm);
        assert_eq!(vm, vec![true, true, true, false, false, true, false, false, false, false]);
    }

    #[test]
    fn given_rows_compute_visibility_right_returns_correct_map() {
        let hm = Matrix::<u8>::from_string("12321\n32111");
        let mut vm = vec![false; hm.vec_size()];
        compute_visibility_right(&hm, &mut vm);
        assert_eq!(vm, vec![false, false, true, true, true, true, true, false, false, true]);
    }

    #[test]
    fn given_cols_compute_visibility_down_returns_correct_map() {
        let hm = Matrix::<u8>::from_string("12\n32\n51");
        let mut vm = vec![false; hm.vec_size()];
        compute_visibility_down(&hm, &mut vm);
        assert_eq!(vm, vec![true, true, true, false, true, false]);
    }

    #[test]
    fn given_cols_compute_visibility_up_returns_correct_map() {
        let hm = Matrix::<u8>::from_string("12\n32\n51");
        let mut vm = vec![false; hm.vec_size()];
        compute_visibility_up(&hm, &mut vm);
        assert_eq!(vm, vec![false, false, false, true, true, true]);
    }

    #[test]
    fn given_cols_and_exisiting_map_compute_visibility_up_returns_correct_map() {
        let hm = Matrix::<u8>::from_string("12\n32\n51");
        let mut vm = vec![true, false, true, false, true, false];
        compute_visibility_up(&hm, &mut vm);
        assert_eq!(vm, vec![true, false, true, true, true, true]);
    }
}
