use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn part1() {
    let file = File::open("inputs/day1/part1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut increased = 0;
    let mut decreased = 0;
    let mut prev_num: Option<u32> = None;

    for line in reader.lines() {
        let num = line
            .expect("Read line correctly")
            .to_string()
            .parse::<u32>()
            .unwrap();
        if prev_num.is_some() {
            if prev_num.unwrap() < num {
                increased += 1;
            } else if prev_num.unwrap() > num {
                decreased += 1;
            }
        }
        prev_num = Some(num);
    }

    println!(
        "Part 1\nIncreased - {}\nDecreased - {}",
        increased, decreased
    );
}

pub fn part2() {
    let file = File::open("inputs/day1/part2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut increased = 0;
    let mut nums = vec![];
    let window = 3;

    for line in reader.lines() {
        let num = line
            .expect("Read line correctly")
            .to_string()
            .parse::<u32>()
            .unwrap();
        nums.push(num);
    }

    let mut prev_num: Option<u32> = None;
    let max = nums.len();
    for start in 0..max {
        let end = cmp::min(start + window, max);
        let current_num: u32 = nums[start..end].iter().sum();

        if prev_num.is_some() {
            if prev_num.unwrap() < current_num {
                increased += 1;
            }
        }

        prev_num = Some(current_num);
    }

    println!("Part2\nIncreased - {}", increased);
}
