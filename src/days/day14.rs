use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

enum QVariant {
    Part1,
    Part2,
}

fn parse_input(
    reader: BufReader<File>,
) -> (HashMap<String, Vec<String>>, HashMap<String, u64>, char) {
    lazy_static! {
        static ref MAPPING: Regex = Regex::new(r"^([A-Z]+) -> ([A-Z])$").unwrap();
    }

    let mut line_iter = reader.lines().peekable();
    let input: Vec<char> = line_iter.next().unwrap().unwrap().chars().collect();
    let mut input_map = HashMap::new();
    for i in 0..input.len() - 1 {
        let pair = format!("{}{}", &input[i], &input[i + 1]);
        let count = input_map.entry(pair).or_insert(0);
        *count += 1;
    }

    line_iter.next();

    /*
        Every pair outputs two pairs per step. We will prebuild the list of
        new pairs that get produced, simplifying things
    */
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in line_iter {
        let line = line.unwrap();

        let captures = MAPPING.captures_iter(line.as_str());

        for cap in captures {
            let pair = cap[1].to_string();
            let output = cap[2].to_string();
            map.insert(
                cap[1].to_string(),
                vec![
                    format!("{}{}", pair.chars().nth(0).unwrap(), output),
                    format!("{}{}", output, pair.chars().nth(1).unwrap()),
                ],
            );
        }
    }

    (map, input_map, input[input.len() - 1])
}

fn step(input: &HashMap<String, u64>, map: &HashMap<String, Vec<String>>) -> HashMap<String, u64> {
    let mut output_map = HashMap::new();

    for (pair, count) in input {
        let new_pairs = map.get(pair).unwrap();

        for new_pair in new_pairs {
            let output_count = output_map.entry(new_pair.to_string()).or_insert(0);
            *output_count += count;
        }
    }

    output_map
}

/*
    We have a map of pairs -> occurrences. But we only want to count the letters.
    Since pairs overlap, we will only count the first character in each pair. Then,
    we will pass in the very last character of the input string, since that's the only char
    that we skip, and it's always at the end.
*/
fn count_most_least_pairs(
    input: &HashMap<String, u64>,
    last_char: char,
) -> ((char, u64), (char, u64)) {
    let mut count_map = HashMap::new();

    let mut most_common: Option<(char, u64)> = None;
    let mut least_common: Option<(char, u64)> = None;
    for (pair, count) in input {
        let pair: Vec<char> = pair.chars().collect();
        for i in 0..pair.len() - 1 {
            *count_map.entry(pair[i].clone()).or_insert(0) += count;
        }
    }

    *count_map.entry(last_char).or_insert(0) += 1;

    for (c, count) in count_map {
        if most_common.is_none() {
            most_common = Some((c, count));
        }

        if count > most_common.unwrap().1 {
            most_common = Some((c, count));
        }

        if least_common.is_none() {
            least_common = Some((c, count));
        }

        if count < least_common.unwrap().1 {
            least_common = Some((c, count));
        }
    }

    (most_common.unwrap(), least_common.unwrap())
}

/*
    Overall idea is to build a map of pair -> occurrences for each step.
    Then, we just need to count how many new occurences of pairs we found, and add the appropriate
    new pairs to our new map.
*/
fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day14/input.txt").unwrap();
    let reader = BufReader::new(file);

    // Keep the last input char for counting
    let (map, mut input, last_char) = parse_input(reader);

    let steps = match variant {
        QVariant::Part1 => 10,
        QVariant::Part2 => 40,
    };

    for _ in 0..steps {
        input = step(&input, &map);
    }

    let (most_common, least_common) = count_most_least_pairs(&input, last_char);

    println!("Most - {} - {}", most_common.0, most_common.1);
    println!("Least - {} - {}", least_common.0, least_common.1);
    println!("Answer - {}", most_common.1 - least_common.1);
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
