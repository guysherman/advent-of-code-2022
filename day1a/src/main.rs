use std::{fs::read_to_string, num::ParseIntError};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let result = count_calories(input.as_str()).unwrap();
    println!("Result: {}", result);
}

fn count_calories(input: &str) -> Result<i32, ParseIntError> {
    let mut input_lines = input.lines();
    let mut current_elf_calories = 0;
    let mut max_calories = 0;

    loop {
        match input_lines.next() {
            Some(expr) => {
                if expr == "" {
                    current_elf_calories = 0;
                } else {
                    let calories = expr.parse::<i32>()?;
                    current_elf_calories += calories;
                    if current_elf_calories > max_calories {
                        max_calories = current_elf_calories
                    }
                }
            }
            None => break,
        };
    }

    Ok(max_calories)
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
        assert_eq!(result, 24000);
    }
}
