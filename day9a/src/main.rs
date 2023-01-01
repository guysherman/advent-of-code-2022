use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let num_positions = count_tail_positions(&input);
    println!("{}", num_positions);

}

struct Simulation {
    head: (i32, i32),
    tail: (i32, i32),
    tail_positions: HashSet<(i32, i32)>,
}

#[derive(Debug, PartialEq, Eq)]
struct Movement {
    amount: i32,
    direction: (i32, i32),
}

impl Movement {
    fn new(amount: i32, direction: (i32, i32)) -> Movement {
        Movement { amount, direction }
    }
}

fn count_tail_positions(input: &str) -> i32 {
    let movements = parse_input(input);
    let mut sim = Simulation::new();
    for m in movements.iter() {
        sim.apply_movement(m);
    }

    sim.tail_positions.len().try_into().unwrap()
}

fn parse_input(input: &str) -> Vec<Movement> {
    input.lines().map(|l| parse_movement(l)).collect()
}

fn parse_movement(line: &str) -> Movement {
    lazy_static! {
        static ref MR: Regex = Regex::new(r"([LRUD]) (\d+)").unwrap();
    }

    let caps = MR.captures(line).unwrap();
    let direction = caps.get(1).unwrap().as_str();
    let amount = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

    match direction {
        "L" => Movement::new(amount, (-1, 0)),
        "R" => Movement::new(amount, (1, 0)),
        "U" => Movement::new(amount, (0, -1)),
        "D" => Movement::new(amount, (0, 1)),
        _ => Movement::new(0, (0, 0)),
    }
}

impl Simulation {
    fn new() -> Simulation {
        let tail_positions = HashSet::from([(0, 0)]);
        Simulation {
            head: (0, 0),
            tail: (0, 0),
            tail_positions,
        }
    }
    fn is_adjacent(&self, pos_tail: (i32, i32)) -> bool {
        let acs = self.adjacent_cells();

        acs.contains(&pos_tail)
    }

    fn adjacent_cells(&self) -> Vec<(i32, i32)> {
        let (x, y) = self.head;
        let template = vec![
            (0, 0),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];
        let result = template.iter().map(|pos| (x + pos.0, y + pos.1)).collect();

        result
    }

    fn calculate_tail_movement(&self) -> (i32, i32) {
        let (x1, y1) = self.head;
        let (x2, y2) = self.tail;

        let mut diffx = x1 - x2;
        if y1 == y2 && diffx.abs() <= 1 {
            diffx = 0;
        }

        let mut diffy = y1 - y2;
        if x1 == x2 && diffy.abs() <= 1 {
            diffy = 0
        }

        (diffx.min(1).max(-1), diffy.min(1).max(-1))
    }

    fn apply_movement(&mut self, movement: &Movement) {
        for _ in 0..movement.amount {
            self.head = (
                self.head.0 + movement.direction.0,
                self.head.1 + movement.direction.1,
            );

            if !self.is_adjacent(self.tail) {
                let tail_movement = self.calculate_tail_movement();
                self.tail = (self.tail.0 + tail_movement.0, self.tail.1 + tail_movement.1);
                self.tail_positions.insert(self.tail);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::RandomState;

    use super::*;

    #[test]
    fn given_0_0_0_0_r4_apply_movement_generates_correct_set() {
        let mut sim = Simulation::new();

        let mov = Movement::new(4, (1, 0));
        sim.apply_movement(&mov);

        let expected: HashSet<(i32, i32), RandomState> =
            HashSet::from_iter(vec![(0, 0), (1, 0), (2, 0), (3, 0)].iter().cloned());
        assert_eq!(sim.tail_positions, expected);
    }

    #[test]
    fn given_r4_parse_movement_returns_4_0() {
        let result = parse_movement("R 4");
        assert_eq!(result, Movement::new(4, (1, 0)));
    }

    #[test]
    fn given_l3_parse_movement_returns_m3_0() {
        let result = parse_movement("L 3");
        assert_eq!(result, Movement::new(3, (-1, 0)));
    }

    #[test]
    fn given_u2_parse_movement_returns_0_m2() {
        let result = parse_movement("U 2");
        assert_eq!(result, Movement::new(2, (0, -1)));
    }

    #[test]
    fn given_d5_parse_movement_returns_0_5() {
        let result = parse_movement("D 5");
        assert_eq!(result, Movement::new(5, (0, 1)));
    }

    static TEST_INPUT: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn given_test_input_parse_input_returns_correct_sequence() {
        let result = parse_input(&TEST_INPUT);
        let expected = vec![
            Movement::new(4, (1, 0)),
            Movement::new(4, (0, -1)),
            Movement::new(3, (-1, 0)),
            Movement::new(1, (0, 1)),
            Movement::new(4, (1, 0)),
            Movement::new(1, (0, 1)),
            Movement::new(5, (-1, 0)),
            Movement::new(2, (1, 0)),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn given_test_input_count_tail_positions_returns_13() {
        let result = count_tail_positions(&TEST_INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn given_2_2_adjacent_cells_returns_correct_results() {
        let sim = Simulation {
            head: (2, 2),
            tail: (2, 2),
            tail_positions: HashSet::new(),
        };

        let result = sim.adjacent_cells();
        let expected = vec![
            (2, 2),
            (2, 1),
            (3, 1),
            (3, 2),
            (3, 3),
            (2, 3),
            (1, 3),
            (1, 2),
            (1, 1),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn given_2_2_1_1_is_adjacent_returns_true() {
        let sim = Simulation {
            head: (2, 2),
            tail: (2, 2),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.is_adjacent((1, 1)), true);
    }

    #[test]
    fn given_2_2_0_0_is_adjacent_returns_false() {
        let sim = Simulation {
            head: (2, 2),
            tail: (2, 2),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.is_adjacent((0, 0)), false);
    }

    #[test]
    fn given_1_1_1_1_movement_returns_0_0() {
        let sim = Simulation {
            head: (1, 1),
            tail: (1, 1),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.calculate_tail_movement(), (0, 0));
    }

    #[test]
    fn given_1_1_0_1_movement_returns_0_0() {
        let sim = Simulation {
            head: (1, 1),
            tail: (0, 1),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.calculate_tail_movement(), (0, 0));
    }

    #[test]
    fn given_1_1_2_1_movement_returns_0_0() {
        let sim = Simulation {
            head: (1, 1),
            tail: (2, 1),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.calculate_tail_movement(), (0, 0));
    }

    #[test]
    fn given_2_2_0_2_movement_returns_1_0() {
        let sim = Simulation {
            head: (2, 2),
            tail: (0, 2),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.calculate_tail_movement(), (1, 0));
    }

    #[test]
    fn given_2_2_4_2_movement_returns_m1_0() {
        let sim = Simulation {
            head: (2, 2),
            tail: (4, 2),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.calculate_tail_movement(), (-1, 0));
    }

    #[test]
    fn given_1_1_1_0_movement_returns_0_0() {
        let sim = Simulation {
            head: (1, 1),
            tail: (1, 0),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.calculate_tail_movement(), (0, 0));
    }

    #[test]
    fn given_1_1_1_2_movement_returns_0_0() {
        let sim = Simulation {
            head: (1, 1),
            tail: (1, 2),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.calculate_tail_movement(), (0, 0));
    }

    #[test]
    fn given_2_2_2_0_movement_returns_0_1() {
        let sim = Simulation {
            head: (2, 2),
            tail: (2, 0),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.calculate_tail_movement(), (0, 1));
    }

    #[test]
    fn given_2_2_2_4_movement_returns_0_m1() {
        let sim = Simulation {
            head: (2, 2),
            tail: (2, 4),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.calculate_tail_movement(), (0, -1));
    }

    #[test]
    fn given_2_2_1_0_movement_returns_1_1() {
        let sim = Simulation {
            head: (2, 2),
            tail: (1, 0),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.calculate_tail_movement(), (1, 1));
    }

    #[test]
    fn given_2_2_0_1_movement_returns_1_1() {
        let sim = Simulation {
            head: (2, 2),
            tail: (0, 1),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.calculate_tail_movement(), (1, 1));
    }

    #[test]
    fn given_2_2_4_1_movement_returns_1_m1() {
        let sim = Simulation {
            head: (2, 2),
            tail: (4, 1),
            tail_positions: HashSet::new(),
        };
        assert_eq!(sim.calculate_tail_movement(), (-1, 1));
    }
}
