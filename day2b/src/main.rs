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

const PLAY_LOSE: char = 'X';
const PLAY_DRAW: char = 'Y';
const PLAY_WIN: char = 'Z';

fn main() {
    let input_text = read_to_string("input.txt").unwrap();
    let result = calculate_scores(&input_text);
    println!("{}", result);
}

fn calculate_scores(input: &str) -> i32 {
    let mut score = 0;

    for play in input.lines() {
        let opponent = shape_to_points(play.chars().nth(0));
        let me = predict_play(opponent, play.chars().nth(2));

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

fn predict_play(opponent_points: i32, key: Option<char>) -> i32 {
    let mut predicted_play = match key {
        Some(PLAY_DRAW) => opponent_points,
        Some(PLAY_WIN) => (opponent_points + 1) % 3,
        Some(PLAY_LOSE) => opponent_points - 1,
        _ => 0,
    };

    if predicted_play == 0 {
        predicted_play = 3
    }

    predicted_play
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
*         W       L       D        W        L
* diff = -2      -1       0        1        2
* +2   =  0       1       2        3        4
* % 3  =  0       1       2        0        1
 *
 */

fn outcome(opponent: i32, me: i32) -> Option<i32> {
    let diff = ((me - opponent) + 2) % 3;

    match diff {
        0 => Some(me + WIN),
        1 => Some(me),
        2 => Some(me + DRAW),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_input_get_twelve() {
        let test_input = r###"
A Y
B X
C Z
        "###
        .trim();

        let result = calculate_scores(test_input);
        assert_eq!(result, 12);
    }

    #[test]
    fn given_rock_and_wind_get_paper() {
        let result = predict_play(ROCK, Some(PLAY_WIN));
        assert_eq!(result, PAPER);
    }

    #[test]
    fn given_rock_and_lose_get_scissors() {
        let result = predict_play(ROCK, Some(PLAY_LOSE));
        assert_eq!(result, SCISSORS);
    }

    #[test]
    fn given_rock_and_draw_get_rock() {
        let result = predict_play(ROCK, Some(PLAY_DRAW));
        assert_eq!(result, ROCK);
    }

    #[test]
    fn given_scissors_and_win_get_rock() {
        let result = predict_play(SCISSORS, Some(PLAY_WIN));
        assert_eq!(result, ROCK);
    }
}
