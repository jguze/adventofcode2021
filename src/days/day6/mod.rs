use std::fs::File;
use std::io::{prelude::*, BufReader};

// Technically + 1 day due to 0 index
const FISH_RESPAWN_DAYS: usize = 6 + 1;
const FISH_NEW_DAYS: usize = 8 + 1;
const MAX_FISH_GROUP: usize = 9;

enum QVariant {
    Part1,
    Part2,
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day6/input.txt").unwrap();
    let reader = BufReader::new(file);

    let starter_fishes: Vec<usize> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut fish_group: [u64; 9] = [0; 9];
    for fish in &starter_fishes {
        fish_group[*fish] += 1;
    }

    let num_days = match variant {
        QVariant::Part1 => 80,
        QVariant::Part2 => 256,
    };

    for i in 0..num_days {
        let curr_day_0 = i % MAX_FISH_GROUP;
        let fish_refreshed = fish_group[curr_day_0];
        fish_group[curr_day_0] = 0;
        fish_group[(curr_day_0 + FISH_RESPAWN_DAYS) % MAX_FISH_GROUP] += fish_refreshed;
        fish_group[(curr_day_0 + FISH_NEW_DAYS) % MAX_FISH_GROUP] += fish_refreshed;
    }

    println!("Answer - {}", fish_group.iter().sum::<u64>());
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
