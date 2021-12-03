use std::fs::File;
use std::io::{prelude::*, BufReader};

enum BinCmp {
    One,
    Zero,
    Equal,
}

fn is_most_common(nums: &Vec<String>, index: usize) -> BinCmp {
    let mut ones = 0;
    let mut zeros = 0;

    for num in nums {
        if num.chars().nth(index).unwrap() == '1' {
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
    let base: u32 = 2;
    let mut value: u32 = 0;

    for i in 0..len {
        let position = len - i - 1;
        if binary_str.chars().nth(i).unwrap() == '1' {
            value += base.pow(position.try_into().unwrap());
        }
    }

    return value;
}

fn retain_by_char(bin_vec: &mut Vec<String>, index: usize, char_to_filer: char) {
    bin_vec.retain(|o| o.chars().nth(index).unwrap() == char_to_filer);
}

pub fn part1() {
    let file = File::open("inputs/day3/part1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut numbers = vec![];

    for line in reader.lines() {
        let text = line.unwrap();
        numbers.push(text);
    }

    if numbers.len() == 0 {
        return;
    }

    let base: i32 = 2;

    let maxlen = numbers[0].len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..maxlen {
        let position = maxlen - i - 1;
        if matches!(is_most_common(&numbers, i), BinCmp::One) {
            gamma += base.pow(position.try_into().unwrap());
        } else {
            epsilon += base.pow(position.try_into().unwrap());
        }
    }

    println!("gamma {}, epsilon {}", gamma, epsilon);
    println!("Answer - {}", gamma * epsilon);
}

pub fn part2() {
    let file = File::open("inputs/day3/part2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut numbers = vec![];

    for line in reader.lines() {
        let text = line.unwrap();
        numbers.push(text);
    }

    if numbers.len() == 0 {
        return;
    }

    let maxlen = numbers[0].len();

    let mut oxygen_vec = numbers.clone();
    let mut co_vec = numbers.clone();

    for i in 0..maxlen {
        if oxygen_vec.len() > 1 {
            match is_most_common(&oxygen_vec, i) {
                BinCmp::One | BinCmp::Equal => {
                    retain_by_char(&mut oxygen_vec, i, '1');
                }
                BinCmp::Zero => {
                    retain_by_char(&mut oxygen_vec, i, '0');
                }
            }
        }

        if co_vec.len() > 1 {
            match is_most_common(&co_vec, i) {
                BinCmp::One | BinCmp::Equal => {
                    retain_by_char(&mut co_vec, i, '0');
                }
                BinCmp::Zero => {
                    retain_by_char(&mut co_vec, i, '1');
                }
            }
        }

        if oxygen_vec.len() == 1 && co_vec.len() == 1 {
            break;
        }
    }

    println!(
        "oxygen {} ({}), co2 {} ({})",
        oxygen_vec[0],
        convert_bin_to_decimal(&oxygen_vec[0]),
        co_vec[0],
        convert_bin_to_decimal(&co_vec[0])
    );
    println!(
        "Answer - {}",
        convert_bin_to_decimal(&oxygen_vec[0]) * convert_bin_to_decimal(&co_vec[0])
    );
}
