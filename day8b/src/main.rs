use std::fs::read_to_string;
mod matrix;
use matrix::Matrix;






fn main() {
    let input = read_to_string("input.txt").unwrap();
    let result = find_best_scenic_score(&input);
    println!("{}", result);
}

fn find_best_scenic_score(input: &str) -> usize {
    // Parse the text into a HeightMap (which is a Vec<u8> plus height and width)
    let hm = Matrix::<u8>::from_string(&input);
    // read through each row forwards and backwards, and then each col forwards and backwards,
    let mut vm = Vec::<usize>::with_capacity(hm.vec_size());

    // work our way through the matrix and work out the scenic score for each point
    for row in 0..hm.height {
        for col in 0..hm.width {
            let score = scenic_score_for_point(&hm, row, col);
            vm.push(score);
        }
    }

    // Find the biggest scenic score
    let mut max_score = 0usize;
    for score in vm.iter() {
        if score > &max_score {
            max_score = *score;
        }
    }

    max_score
}

fn scenic_score_for_point(hm: &Matrix<u8>, row_index: usize, column_index: usize) -> usize {
    let index = hm.index_from_point(column_index, row_index);
    let height = hm.data[index];

    let mut result = 1;

    result *= count_trees_in_direction(height, hm.left_iter(row_index, column_index));
    result *= count_trees_in_direction(height, hm.right_iter(row_index, column_index));
    result *= count_trees_in_direction(height, hm.down_iter(row_index, column_index));
    result *= count_trees_in_direction(height, hm.up_iter(row_index, column_index));

    result
}

fn count_trees_in_direction<'a, I>(height: u8, dir_iter: I) -> usize
where
 I: Iterator<Item = &'a u8>
{
    let mut count = 0usize;
    for h in dir_iter {
        count += 1;
        if *h >= height {
            break
        }
    }

    count
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
    fn given_test_input_best_score_is_8() {
        let result = find_best_scenic_score(&INPUT);
        assert_eq!(result, 8);
    }

    #[test]
    fn given_test_input_scenic_score_for_2_2_returns_2() {
        let hm = Matrix::from_string(&INPUT);
        let result = scenic_score_for_point(&hm, 2, 2);

        assert_eq!(result, 1);
    }

}
