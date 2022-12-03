use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let test_input = read_to_string("input.txt").unwrap();
    let result = sum_priorities(&test_input);
    println!("{}", result);
}

fn sum_priorities(input: &str) -> u32 {
    let mut total_priority = 0;
    // for rucksacks
    for rucksack in input.lines() {
        //      find shared item
        let shared_item: char = find_shared_items(&rucksack);
        //      get item priority
        let item_priority: u32 = get_item_priority(shared_item);
        //      sum ++
        total_priority += item_priority;
    }
    total_priority
}

fn find_shared_items(rucksack: &str) -> char {
    let split = rucksack.len() / 2;
    let left = &rucksack[..split];
    let right = &rucksack[split..];

    let mut left_set = HashSet::new();
    for letter in left.chars() {
        left_set.insert(letter);
    }

    for letter in right.chars() {
        if left_set.contains(&letter) {
            return letter;
        }
    }

    0 as char
}

const ASCII_LOWER_A: u32 = 'a' as u32;
const ASCII_LOWER_Z: u32 = 'z' as u32;
const ASCII_UPPER_A: u32 = 'A' as u32;
const ASCII_UPPER_Z: u32 = 'Z' as u32;

fn get_item_priority(shared_item: char) -> u32 {
    match shared_item as u32 {
        n if n >= ASCII_LOWER_A && n <= ASCII_LOWER_Z => (n - ASCII_LOWER_A) + 1,
        n if n >= ASCII_UPPER_A && n <= ASCII_UPPER_Z => (n - ASCII_UPPER_A) + 27,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_input_then_get_157() {
        let test_input = r###"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"###
            .trim();

        let result = sum_priorities(test_input);
        assert_eq!(result, 157);
    }

    #[test]
    fn given_case_1_then_find_shared_items_returns_p() {
        let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let result = find_shared_items(&test_input);
        assert_eq!(result, 'p');
    }

    #[test]
    fn given_case_2_then_find_shared_items_returns_upper_l() {
        let test_input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let result = find_shared_items(&test_input);
        assert_eq!(result, 'L');
    }

    #[test]
    fn given_case_3_then_find_shared_items_returns_upper_p() {
        let test_input = "PmmdzqPrVvPwwTWBwg";
        let result = find_shared_items(&test_input);
        assert_eq!(result, 'P');
    }

    #[test]
    fn given_case_4_then_find_shared_items_returns_v() {
        let test_input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let result = find_shared_items(&test_input);
        assert_eq!(result, 'v');
    }

    #[test]
    fn given_case_5_then_find_shared_items_returns_t() {
        let test_input = "ttgJtRGJQctTZtZT";
        let result = find_shared_items(&test_input);
        assert_eq!(result, 't');
    }

    #[test]
    fn given_case_6_then_find_shared_items_returns_s() {
        let test_input = "CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = find_shared_items(&test_input);
        assert_eq!(result, 's');
    }

    #[test]
    fn given_a_then_get_item_priority_returns_1() {
        let result = get_item_priority('a');
        assert_eq!(result, 1);
    }

    #[test]
    fn given_z_then_get_item_priority_returns_26() {
        let result = get_item_priority('z');
        assert_eq!(result, 26);
    }

    #[test]
    fn given_upper_a_then_get_item_priority_returns_27() {
        let result = get_item_priority('A');
        assert_eq!(result, 27);
    }

    #[test]
    fn given_upper_z_then_get_item_priority_returns_52() {
        let result = get_item_priority('Z');
        assert_eq!(result, 52);
    }
}
