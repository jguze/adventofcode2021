use std::fs::File;
use std::io::{prelude::*, BufReader};

enum BinCmp {
    One,
    Zero,
    Equal,
}

fn is_most_common(nums: &Vec<u32>, bit_pos: usize) -> BinCmp {
    let mut ones = 0;
    let mut zeros = 0;
    let bit = 1 << bit_pos;

    for num in nums {
        if num & bit == bit {
            ones += 1;
        } else {
            zeros += 1;
        }
    }

    if ones > zeros {
        return BinCmp::One;
    } else if zeros > ones {
        return BinCmp::Zero;
    } else {
        return BinCmp::Equal;
    }
}

fn convert_bin_to_decimal(binary_str: &str) -> u32 {
    let len = binary_str.len();
    let mut value: u32 = 0;

    for i in 0..len {
        let position = len - i - 1;
        if binary_str.chars().nth(i).unwrap() == '1' {
            value += 1 << position;
        }
    }

    return value;
}

fn retain_by_bit(bin_vec: &mut Vec<u32>, bit_pos: usize, bit_value: u8) {
    let bit = 1 << bit_pos;
    let comparator = if bit_value == 1 { bit } else { 0 };
    bin_vec.retain(|o| o & bit == comparator);
}

pub fn part1() {
    let file = File::open("inputs/day3/part1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut numbers = vec![];

    let mut maxlen: Option<usize> = None;

    for line in reader.lines() {
        let text = line.unwrap();
        numbers.push(convert_bin_to_decimal(&text));

        if maxlen.is_none() {
            maxlen = Some(text.len());
        }
    }

    if numbers.len() == 0 {
        return;
    }

    let maxlen = maxlen.unwrap();
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..maxlen {
        let bit_position = maxlen - i - 1;
        if matches!(is_most_common(&numbers, bit_position), BinCmp::One) {
            gamma += 1 << bit_position;
        } else {
            epsilon += 1 << bit_position;
        }
    }

    println!("gamma {}, epsilon {}", gamma, epsilon);
    println!("Answer - {}", gamma * epsilon);
}

pub fn part2() {
    let file = File::open("inputs/day3/part2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut numbers = vec![];
    let mut maxlen: Option<usize> = None;

    for line in reader.lines() {
        let text = line.unwrap();
        numbers.push(convert_bin_to_decimal(&text));

        if maxlen.is_none() {
            maxlen = Some(text.len());
        }
    }

    if numbers.len() == 0 {
        return;
    }

    let mut oxygen_vec = numbers.clone();
    let mut co_vec = numbers.clone();
    let maxlen = maxlen.unwrap();

    for i in 0..maxlen {
        let bit_position = maxlen - i - 1;
        if oxygen_vec.len() > 1 {
            match is_most_common(&oxygen_vec, bit_position) {
                BinCmp::One | BinCmp::Equal => {
                    retain_by_bit(&mut oxygen_vec, bit_position, 1);
                }
                BinCmp::Zero => {
                    retain_by_bit(&mut oxygen_vec, bit_position, 0);
                }
            }
        }

        if co_vec.len() > 1 {
            match is_most_common(&co_vec, bit_position) {
                BinCmp::One | BinCmp::Equal => {
                    retain_by_bit(&mut co_vec, bit_position, 0);
                }
                BinCmp::Zero => {
                    retain_by_bit(&mut co_vec, bit_position, 1);
                }
            }
        }

        if oxygen_vec.len() == 1 && co_vec.len() == 1 {
            break;
        }
    }

    println!("oxygen ({}), co2 ({})", oxygen_vec[0], co_vec[0],);
    println!("Answer - {}", oxygen_vec[0] * &co_vec[0]);
}
