use std::fs::File;
use std::io::{prelude::*, BufReader};

const FISH_RESPAWN_DAYS: u32 = 6;
const FISH_NEW_DAYS: u32 = 8;

enum QVariant {
    Part1,
    Part2,
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day6/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut fishes: Vec<u32> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let num_days = match variant {
        QVariant::Part1 => 80,
        QVariant::Part2 => 256,
    };

    for i in 0..num_days {
        println!("Day - {}", i);
        let mut new_fishes = 0;
        for fish in &mut fishes {
            if *fish == 0 {
                *fish = FISH_RESPAWN_DAYS;
                new_fishes += 1;
            } else {
                *fish -= 1;
            }
        }

        while new_fishes > 0 {
            fishes.push(FISH_NEW_DAYS);
            new_fishes -= 1;
        }
    }

    println!("Answer - {}", fishes.len());
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
