use itertools::zip;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

enum QVariant {
    Part1,
    Part2,
}

fn sort_string(s: &str) -> String {
    let mut char_arr = s.chars().collect::<Vec<char>>();
    char_arr.sort_by(|a, b| b.cmp(a));
    String::from_iter(char_arr)
}

fn map_signals(signals: &Vec<&str>) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for signal in signals {
        let sorted_signal = sort_string(signal);
        match sorted_signal.len() {
            2 => {
                map.insert(sorted_signal, 1);
            }
            4 => {
                map.insert(sorted_signal, 4);
            }
            3 => {
                map.insert(sorted_signal, 7);
            }
            7 => {
                map.insert(sorted_signal, 8);
            }
            _ => {
                // pass
            }
        }
    }

    map
}

fn slot_signals(signals: &Vec<&str>, map: &mut HashMap<String, u32>) {
    // filter out 1, 4, 7 ,8
    let unknown_values: Vec<String> = signals
        .iter()
        .filter(|s| match s.len() {
            2 | 3 | 4 | 7 => false,
            _ => true,
        })
        .map(|x| x.to_string())
        .collect();

    let mut char_to_segment_map: HashMap<char, u32> = HashMap::new();

    let mut known_values: Vec<(&String, &u32)> = map.iter().collect();

    let mut unknown_map: HashMap<String, u32> = HashMap::new();

    let result = slot_values(
        &mut char_to_segment_map,
        &mut known_values,
        &unknown_values,
        &mut unknown_map,
    );

    assert_eq!(result, true);

    // Annoying borrowing workaround
    for (key, value) in unknown_map {
        map.insert(key, value);
    }
}

// The only point of this is to see if we can fill segments given a permutation
fn try_fill_clock_with_chars(
    char_to_segment_map: &mut HashMap<char, u32>,
    pairs: &Vec<(&char, &u32)>,
) -> bool {
    let mut entries = vec![];
    for (c, i) in pairs {
        match char_to_segment_map.entry(**c) {
            Entry::Vacant(o) => {
                o.insert(**i);
                entries.push(c);
            }
            Entry::Occupied(o) => {
                if *o.get() != **i {
                    // This will not work. Rewind and abort
                    for entry in entries {
                        char_to_segment_map.remove(entry);
                    }

                    return false;
                }
            }
        }
    }

    true
}

fn remove_segments_from_map(char_to_segment_map: &mut HashMap<char, u32>, chars: &Vec<char>) {
    for c in chars {
        char_to_segment_map.remove(c);
    }
}

fn compare_vecs(a: &Vec<u32>, b: &Vec<u32>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }

    true
}

fn compare_all_vecs(a: &Vec<u32>, b: &[Vec<u32>; 10]) -> Option<u32> {
    for (e, other_seg) in b.iter().enumerate() {
        if compare_vecs(a, other_seg) {
            return Some(e as u32);
        }
    }

    None
}

fn slot_values(
    char_to_segment_map: &mut HashMap<char, u32>,
    known_values: &mut Vec<(&String, &u32)>,
    unknown_values: &Vec<String>,
    char_num_map: &mut HashMap<String, u32>,
) -> bool {
    /*
      If you treat every edge like a number, then each number has this mapping
       0
      1 2
       3
      4 5
       6

       0 - 0, 1, 2, 4, 5, 6
       1 - 2, 5
       2 - 0, 2, 3, 4, 6
       3 - 0, 2, 3, 5, 6
       4 - 1, 2, 3, 5
       5 - 0, 1, 3, 5, 6
       6 - 0, 1, 3, 4, 5, 6
       7 - 0, 2, 5
       8 - 0, 1, 2, 3, 4, 5, 6
       9 - 0, 1, 2, 3, 5, 6
    */
    lazy_static! {
        static ref SEGMENT_FOR_NUM: [Vec<u32>; 10] = [
            vec![0, 1, 2, 4, 5, 6],
            vec![2, 5],
            vec![0, 2, 3, 4, 6],
            vec![0, 2, 3, 5, 6],
            vec![1, 2, 3, 5],
            vec![0, 1, 3, 5, 6],
            vec![0, 1, 3, 4, 5, 6],
            vec![0, 2, 5],
            vec![0, 1, 2, 3, 4, 5, 6],
            vec![0, 1, 2, 3, 5, 6],
        ];
    }

    if known_values.len() == 0 {
        // Have to try to fit the unknown values, to really make sure this permutation is correct
        let mut pairs = vec![];
        for unknown in unknown_values {
            let mut segs = vec![];
            for c in unknown.chars() {
                segs.push(char_to_segment_map.get(&c).unwrap().clone());
            }

            segs.sort();
            let number = compare_all_vecs(&segs, &SEGMENT_FOR_NUM);
            if number.is_none() {
                return false;
            } else {
                pairs.push((unknown, number.unwrap().clone()));
            }
        }

        for pair in pairs {
            char_num_map.insert(sort_string(pair.0), pair.1);
        }

        return true;
    }

    let (current_str, current_num) = known_values.pop().unwrap();

    let current_chars = current_str.chars().collect::<Vec<char>>();
    let positions = &SEGMENT_FOR_NUM[*current_num as usize];

    /*
        We are going to try to recursively map segments to a number, for all possible permutations
        for the known values (1, 4, 7, 8).
        Since there's multiple ways this can work, we then need to try to fit the unknown values.

        At the end, there will be one possible mapping
    */
    for perm in positions.iter().permutations(positions.len()) {
        if try_fill_clock_with_chars(char_to_segment_map, &zip(&current_chars, perm).collect()) {
            if slot_values(
                char_to_segment_map,
                known_values,
                unknown_values,
                char_num_map,
            ) {
                return true;
            } else {
                remove_segments_from_map(char_to_segment_map, &current_chars);
            }
        }
    }

    /*
        d - 0
        e - 1
        a - 2
        f - 3
        g - 4
        b - 5
        c - 6

        ddd
       f   b
       f   b
        eee
       g   a
       g   a
        ccc
    */
    known_values.push((current_str, current_num));
    false
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day8/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let input = line.unwrap();
        let tokens: Vec<&str> = input.split('|').collect();

        let input_signals: Vec<&str> = tokens[0].split_whitespace().collect();
        let display_signals: Vec<&str> = tokens[1].split_whitespace().collect();

        let mut map = map_signals(&input_signals);

        if matches!(variant, QVariant::Part2) {
            slot_signals(&input_signals, &mut map);
        }

        let base: i32 = 10;
        for (e, signal) in display_signals.iter().enumerate() {
            let size = display_signals.len();
            let digit = (size - e - 1) as u32;
            let key = &sort_string(signal);
            if map.contains_key(&sort_string(signal)) {
                match variant {
                    QVariant::Part1 => {
                        count += 1;
                    }
                    QVariant::Part2 => {
                        count += map.get(key).unwrap() * (base.pow(digit) as u32);
                    }
                }
            }
        }
    }

    println!("Answer - {}", count);
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
