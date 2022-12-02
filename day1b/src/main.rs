use std::{fs::read_to_string, num::ParseIntError};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let result = count_calories(input.as_str()).unwrap();
    println!("Result: {}", result);
}

fn count_calories(input: &str) -> Result<i32, ParseIntError> {
    let mut input_lines = input.lines();
    let mut sum = 0;
    let mut most = 0;
    let mut second_most = 0;
    let mut third_most = 0;

    loop {
        match input_lines.next() {
            Some("") => {
                update_top_three(&mut sum, &mut most, &mut second_most, &mut third_most);
            }
            Some(expr) => {
                let calories = expr.parse::<i32>()?;
                sum += calories;
            }
            None => {
                update_top_three(&mut sum, &mut most, &mut second_most, &mut third_most);
                break;
            }
        };
    }

    Ok(most + second_most + third_most)
}

fn update_top_three(sum: &mut i32, most: &mut i32, second_most: &mut i32, third_most: &mut i32) {
    if sum > most {
        *third_most = *second_most;
        *second_most = *most;
        *most = *sum;
    } else if sum > second_most {
        *third_most = *second_most;
        *second_most = *sum;
    } else if sum > third_most {
        *third_most = *sum;
    }
    *sum = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_input_get_24000() {
        let test_input = r###"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"###
        .trim();

        let result = count_calories(test_input).unwrap();
        assert_eq!(result, 45000);
    }
}
