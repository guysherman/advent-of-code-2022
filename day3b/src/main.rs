use std::{
    collections::{hash_map::RandomState, HashSet},
    fs::read_to_string,
    str::Lines,
};

fn main() {
    let test_input = read_to_string("input.txt").unwrap();
    let result = sum_priorities(&test_input);
    println!("{}", result);
}

fn sum_priorities(input: &str) -> u32 {
    let mut total_priority = 0;
    let mut rucksacks = input.lines();
    // for rucksacks
    loop {
        let group = get_iterator_chunk(&mut rucksacks);
        if group.len() < 3 {
            break;
        }
        //      find shared item
        let shared_item: char = find_shared_items(group.as_slice());
        //      get item priority
        let item_priority: u32 = get_item_priority(shared_item);
        //      sum ++
        total_priority += item_priority;
    }
    total_priority
}

fn get_iterator_chunk<'a>(iterator: &'a mut Lines) -> Vec<&'a str> {
    let mut result = Vec::new();
    for _ in 0..3 {
        match iterator.next() {
            Some(l) => result.push(l),
            None => break,
        }
    }

    result
}

fn find_shared_items(rucksacks: &[&str]) -> char {
    if rucksacks.len() != 3 {
        panic!();
    }

    let a: HashSet<char, RandomState> = HashSet::from_iter(rucksacks.get(0).unwrap().chars());
    let b: HashSet<char, RandomState> = HashSet::from_iter(rucksacks.get(1).unwrap().chars());
    let c: HashSet<char, RandomState> = HashSet::from_iter(rucksacks.get(2).unwrap().chars());

    let a_b: HashSet<char> = Iterator::collect::<HashSet<_>>(Iterator::cloned(a.intersection(&b)));
    let a_b_c: HashSet<char> =
        Iterator::collect::<HashSet<_>>(Iterator::cloned(a_b.intersection(&c)));

    *a_b_c.iter().next().unwrap()
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
    fn given_test_input_then_get_70() {
        let test_input = r###"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"###
            .trim();

        let result = sum_priorities(test_input);
        assert_eq!(result, 70);
    }

    #[test]
    fn given_scenario_1_then_find_shared_items_returns_r() {
        let binding = r###"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg"###
            .lines()
            .collect::<Vec<_>>();
        let test_input = binding.as_slice();

        let result = find_shared_items(test_input);
        assert_eq!(result, 'r');
    }

    #[test]
    fn given_scenario_1_then_find_shared_items_returns_upper_z() {
        let binding = r###"vwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"###
            .lines()
            .collect::<Vec<_>>();
        let test_input = binding.as_slice();

        let result = find_shared_items(test_input);
        assert_eq!(result, 'Z');
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
