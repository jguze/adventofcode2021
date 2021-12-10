use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

enum QVariant {
    Part1,
    Part2,
}

fn find_illegal_closing_and_complete(
    input: &str,
    complement_map: &HashMap<char, char>,
) -> (Option<char>, Option<Vec<char>>) {
    let mut stack = vec![];
    for c in input.chars() {
        if complement_map.contains_key(&c) {
            stack.push(complement_map.get(&c).unwrap().clone());
        } else {
            let maybe_expected = stack.pop();
            if maybe_expected.is_none() {
                panic!("This should not be possible!");
            }

            let expected = maybe_expected.unwrap();
            if c != expected {
                return (Some(c), None);
            }
        }
    }

    (None, Some(stack))
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day10/input.txt").unwrap();
    let reader = BufReader::new(file);

    let complement_map: HashMap<char, char> = [('{', '}'), ('[', ']'), ('(', ')'), ('<', '>')]
        .into_iter()
        .collect();
    let illegal_score_map: HashMap<char, u64> = [('}', 1197), (']', 57), (')', 3), ('>', 25137)]
        .into_iter()
        .collect();
    let missing_score_map: HashMap<char, u64> = [('}', 3), (']', 2), (')', 1), ('>', 4)]
        .into_iter()
        .collect();

    let mut illegal_closing_score = 0;
    let mut missing_closing_scores_vec = vec![];
    for line in reader.lines() {
        let mut missing_closing_score = 0;
        let input = line.unwrap();
        let (maybe_illegal, maybe_missing_close_chars) =
            find_illegal_closing_and_complete(&input, &complement_map);

        if maybe_illegal.is_some() {
            illegal_closing_score += illegal_score_map.get(&maybe_illegal.unwrap()).unwrap();
        } else {
            let mut missing_close_chars = maybe_missing_close_chars.unwrap();
            while let Some(c) = missing_close_chars.pop() {
                missing_closing_score =
                    (missing_closing_score * 5) + missing_score_map.get(&c).unwrap();
            }

            missing_closing_scores_vec.push(missing_closing_score);
        }
    }

    match variant {
        QVariant::Part1 => {
            println!("Answer - {}", illegal_closing_score);
        }
        QVariant::Part2 => {
            missing_closing_scores_vec.sort();
            println!(
                "Answer - {}",
                missing_closing_scores_vec[missing_closing_scores_vec.len() / 2]
            );
        }
    }
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
