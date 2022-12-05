use std::{fs::read_to_string, ops::RangeInclusive};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct RangePair {
    left: RangeInclusive<u32>,
    right: RangeInclusive<u32>,
}

impl RangePair {
    fn is_fully_containing(&self) -> bool {
        self.left.fully_contains(&self.right) || self.right.fully_contains(&self.left)
    }
}

trait FullyContains<Rhs=Self> {
    fn fully_contains(&self, rhs: &Rhs) -> bool;
}


impl FullyContains for RangeInclusive<u32> {
    fn fully_contains(&self, rhs: &Self) -> bool {
        return self.start() <= rhs.start() && self.end() >= rhs.end();
    }
}

trait Overlaps<Rhs=Self> {
    fn overlaps(&self, rhs: &Rhs) -> bool;
}

impl Overlaps for RangeInclusive<u32> {
    fn overlaps(&self, rhs: &Self) -> bool {
        return (self.start() <= rhs.start() && self.end() >= rhs.start())
            || (rhs.start() <= self.start() && rhs.end() >= self.start());
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ResultValues {
    contained: u32,
    overlaps: u32
}

fn main() {
    let input_text = read_to_string("input.txt").unwrap();
    let result = count_fully_contained(&input_text);
    println!("{}, {}", &result.contained, &result.overlaps);
}


fn count_fully_contained(input: &str) -> ResultValues {
    let mut fully_contained_pairs = 0;
    let mut overlaps = 0;
    // Parse lines into pairs of ranges
    for line in input.lines() {
        let range_pair = line_to_ranges(line);
        // Compare ranges for overlap
        // if overlap sum++
        if range_pair.is_fully_containing() {
            fully_contained_pairs += 1;
        }

        if range_pair.left.overlaps(&range_pair.right) {
            overlaps += 1;
        }
    }
    
    ResultValues {
        contained: fully_contained_pairs,
        overlaps
    }
}


fn line_to_ranges(line: &str) -> RangePair {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }   

    let captures = RE.captures(line).unwrap();
    let left_lower = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let left_upper = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let right_lower = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
    let right_upper = captures.get(4).unwrap().as_str().parse::<u32>().unwrap();

    RangePair {
        left: (left_lower..=left_upper),
        right: (right_lower..=right_upper),
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_input_returns_2_4() {
        let test_input = r###"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"###.trim();
        let result = count_fully_contained(test_input);

        assert_eq!(result, ResultValues {
            contained: 2,
            overlaps: 4
        });
    }

    #[test]
    fn given_all_single_digits_then_returns_correct_pair() {
        let test_input = r"1-2,3-4";
        let expected = RangePair {
            left: (1..=2),
            right: (3..=4)
        };

        let result = line_to_ranges(&test_input);
        assert_eq!(expected, result);
    }

    #[test]
    fn given_all_double_digits_then_returns_correct_pair() {
        let test_input = r"11-12,13-14";
        let expected = RangePair {
            left: (11..=12),
            right: (13..=14)
        };

        let result = line_to_ranges(&test_input);
        assert_eq!(expected, result);

    }

    #[test]
    fn given_fully_contained_ranges_return_true() {
        let pair = RangePair {
            left: (1..=5),
            right: (2..=3),
        };
        let result = pair.is_fully_containing();
        assert_eq!(result, true);
    }

    #[test]
    fn given_revers_fully_contained_ranges_is_fully_containing_returns_return_true() {
        let pair = RangePair {
            left: (2..=3),
            right: (1..=5),
        };
        let result = pair.is_fully_containing();
        assert_eq!(result, true);
    }

    #[test]
    fn given_mutually_exclusive_ranges_is_fully_containing_returns_false() {
        let pair = RangePair {
            left: (1..=2),
            right: (3..=4)
        };
        let result = pair.is_fully_containing();
        assert_eq!(result, false);
    }

    #[test]
    fn given_overlapping_ranges_is_fully_containing_returns_false() {
        let pair = RangePair {
            left: (1..=3),
            right: (2..=4)
        };
        let result = pair.is_fully_containing();
        assert_eq!(result, false);
    }

    #[test]
    fn given_right_higher_overlaps_left_lower_overlaps_returns_true() {
        assert!((1..=3).overlaps(&(2..=4)));
    }
    
    #[test]
    fn given_right_lower_overlaps_left_higher_overlaps_returns_true() {
        assert!((2..=4).overlaps(&(1..=3)));
    }
}
