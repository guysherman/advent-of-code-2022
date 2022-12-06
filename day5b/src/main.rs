use std::fs::read_to_string;

use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq)]
struct MoveInstruction {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct PuzzleInput<'a> {
    stack_definition: &'a [&'a str],
    moves_definition: &'a [&'a str],
}

fn main() {
    let test_input = read_to_string("input.txt").unwrap();
    let top_sequence = determine_top_sequence(&test_input);
    println!("{}", top_sequence);
}

fn determine_top_sequence(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();

    let puzzle_input = get_puzzle_input(&lines);
    // Parse stacks
    let mut stacks = parse_stacks(&puzzle_input.stack_definition);
    // Parse movements
    handle_movements(&mut stacks, &puzzle_input.moves_definition);
    
    // Read top of stats
    read_top_of_stacks(&stacks)
}

fn get_puzzle_input<'a>(lines: &'a Vec<&'a str>) -> PuzzleInput<'a> {
    let mut split: usize = 0;
    for (i, line) in lines.iter().enumerate() {
        if *line == "" {
            split = i;
            break;
        }
    }

    let puzzle_input = PuzzleInput {
        stack_definition : &lines[..split],
        moves_definition : &lines[split+1..],
    };

    puzzle_input
}

fn parse_stacks<'a>(lines: &[&'a str]) -> Vec<Vec<&'a str>> {
    // Set up the vec of vecs
    let mut stacks = provision_stacks(lines);
    // Load up the stacks
    load_stacks(&lines[..lines.len()], &mut stacks);

    stacks
}

fn provision_stacks<'a>(rows: &[&'a str]) -> Vec::<Vec::<&'a str>> {
    let mut stacks = Vec::<Vec::< &str>>::new();
    let label_row = rows.last().unwrap();
    let label_regex = Regex::new(r"\d+").unwrap();
    for _ in label_regex.find_iter(label_row) {
        stacks.push(Vec::with_capacity(rows.len()));
    }

    stacks
}

fn load_stacks<'a>(rows: &[&'a str], stacks: &mut Vec::<Vec::<&'a str>>) {
    let stack_item_regex = Regex::new(r"(   |\[([A-Z])\]) ?").unwrap();
    for row in rows.iter().rev() {
        for (i, caps) in stack_item_regex.captures_iter(row).enumerate() {
            if caps.len() >= 2 {
                match &caps.get(2) {
                    Some(letter) => stacks.get_mut(i).unwrap().push(letter.as_str()),
                    None => continue,
                }
            }
        }

    }
}

fn handle_movements<'a>(stacks: &mut Vec::<Vec::<&'a str>>, lines: &[&'a str]) {
    for row in lines.iter() {
        let move_instruction = parse_move(row);
        process_move(stacks, move_instruction);
    }
}

fn parse_move(move_string: &str) -> MoveInstruction {
    lazy_static! {
        static ref MOVE_REGEX: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    let captures = MOVE_REGEX.captures(move_string).unwrap();
    let count = &captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let from = &captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
    let to = &captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

    MoveInstruction { count: *count, from, to }
}

fn process_move<'a>(stacks: &mut Vec::<Vec::<&'a str>>, move_instruction: MoveInstruction) {
    let from = &mut stacks[move_instruction.from];
    let mut to_move = from.split_off(from.len() - move_instruction.count);
    let to: &mut Vec<&str> = &mut stacks[move_instruction.to];
    to.append(&mut to_move);
}

fn read_top_of_stacks<'a>(stacks: &'a Vec::<Vec::<&'a str>>) -> String {
    let mut result = String::with_capacity(stacks.len());
    for stack in stacks {
        result.push(stack.last().unwrap().chars().nth(0).unwrap());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "\x20   [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

    #[test]
    fn given_test_input_returns_mcd() {
        let result = determine_top_sequence(&TEST_INPUT);
        assert_eq!(result, "MCD");
    }

    #[test]
    fn given_test_input_get_puzzle_input_returns_correct_struct() {
        let lines = TEST_INPUT.lines().collect::<Vec<&str>>();
        let puzzle_input = get_puzzle_input(&lines);
        let expected = PuzzleInput {
            stack_definition: &lines[..4],
            moves_definition: &lines[5..],
        };

        assert_eq!(puzzle_input, expected);
    }

    #[test]
    fn given_test_input_parse_stacks_returns_three_stacks() {
        let lines = TEST_INPUT.lines().collect::<Vec<&str>>();
        let puzzle_input = get_puzzle_input(&lines);
        let result = parse_stacks(&puzzle_input.stack_definition);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn given_test_input_parse_stacks_input_returns_stack_with_zn() {
        let lines = TEST_INPUT.lines().collect::<Vec<&str>>();
        let puzzle_input = get_puzzle_input(&lines);
        let result = parse_stacks(&puzzle_input.stack_definition);
        assert_eq!(result[0], vec!["Z", "N"]);
    }

    #[test]
    fn given_test_input_parse_stacks_input_returns_stack_with_mcd() {
        let lines = TEST_INPUT.lines().collect::<Vec<&str>>();
        let puzzle_input = get_puzzle_input(&lines);
        let result = parse_stacks(&puzzle_input.stack_definition);
        assert_eq!(result[1], vec!["M", "C", "D"]);
    }

    #[test]
    fn given_test_input_parse_stacks_input_returns_stack_with_p() {
        let lines = TEST_INPUT.lines().collect::<Vec<&str>>();
        let puzzle_input = get_puzzle_input(&lines);
        let result = parse_stacks(&puzzle_input.stack_definition);
        assert_eq!(result[2], vec!["P"]);
    }

    #[test]
    fn given_single_move_process_move_returns_stacks_with_zn_mc_pd() {
        let mut stacks = vec![
            vec!["Z", "N"],
            vec!["M", "C", "D"],
            vec!["P"],
        ];

        let expected = vec![
            vec!["Z", "N"],
            vec!["M", "C"],
            vec!["P", "D"],
        ];
 
        process_move(&mut stacks, MoveInstruction {
            count: 1,
            from: 1,
            to: 2,
        });

        assert_eq!(stacks, expected);
    }

    #[test]
    fn given_multi_move_process_move_returns_stacks_with_zn_m_pcd() {
        let mut stacks = vec![
            vec!["Z", "N"],
            vec!["M", "C", "D"],
            vec!["P"],
        ];

        let expected = vec![
            vec!["Z", "N"],
            vec!["M"],
            vec!["P", "C", "D"],
        ];
 
        process_move(&mut stacks, MoveInstruction {
            count: 2,
            from: 1,
            to: 2,
        });

        assert_eq!(stacks, expected);
    }

    #[test]
    fn given_multi_digits_parse_move_returns_move_instruction() {
        let move_string = "move 20 from 11 to 14";
        let expected = MoveInstruction {
            count: 20,
            from: 10,
            to: 13,
        };

        let result = parse_move(&move_string);
        assert_eq!(result, expected);
    }

}
