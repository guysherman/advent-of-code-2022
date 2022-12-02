use std::fs::read_to_string;

const WIN: i32 = 6;
const DRAW: i32 = 3;
const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const OPPONENT_ROCK: char = 'A';
const OPPONENT_PAPER: char = 'B';
const OPPONENT_SCISSORS: char = 'C';

const ME_ROCK: char = 'X';
const ME_PAPER: char = 'Y';
const ME_SCISSORS: char = 'Z';

fn main() {
    let input_text = read_to_string("input.txt").unwrap();
    let result = calculate_scores(&input_text);
    println!("{}", result);
}

fn calculate_scores(input: &str) -> i32 {
    let mut score = 0;

    for play in input.lines() {
        let opponent = shape_to_points(play.chars().nth(0));
        let me = shape_to_points(play.chars().nth(2));

        if opponent == 0 || me == 0 {
            continue;
        }

        score += outcome(opponent, me).unwrap();
    }

    score
}

fn shape_to_points(shape: Option<char>) -> i32 {
    match shape {
        Some(OPPONENT_ROCK) | Some(ME_ROCK) => ROCK,
        Some(OPPONENT_PAPER) | Some(ME_PAPER) => PAPER,
        Some(OPPONENT_SCISSORS) | Some(ME_SCISSORS) => SCISSORS,
        Some(_expr) => 0,
        None => 0,
    }
}

/*
* a little bit of math
* 2 - 1 = 1 => WIN
* 3 - 2 = 1 => WIN
* 3 - 1 = 2 => LOSE
* 1 - 2 = -1 => LOSE
* 2 - 3 = -1 => LOSE
* 1 - 3 = -2 => WIN
*
* diff = me - opponent
* diff == -2 || diff == 1 => WIN
* diff == 2 || diff == -1 => LOSE
* diff == 0 => DRAW
 *
 */

fn outcome(opponent: i32, me: i32) -> Option<i32> {
    let diff = me - opponent;

    match diff {
        -2 | 1 => Some(me + WIN),
        0 => Some(me + DRAW),
        2 | -1 => Some(me),
        _i => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_input_get_fifteen() {
        let test_input = r###"
A Y
B X
C Z
        "###
        .trim();

        let result = calculate_scores(test_input);
        assert_eq!(result, 15);
    }
}
