use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

enum QVariant {
    Part1,
    Part2,
}

fn move_to(
    positions: &Vec<usize>,
    goal: usize,
    variant: &QVariant,
    lookup_map: &mut HashMap<usize, i32>,
) -> u32 {
    let mut fuel = 0;

    for pos in positions {
        match variant {
            QVariant::Part1 => fuel += (goal as i32 - *pos as i32).abs(),
            QVariant::Part2 => {
                // micro optimization using lookup table.
                // Without this, Debug builds take 20+ seconds, and release takes 0.7s
                // Adding the lookup makes it run in 1.5s in Debug, and 0.1s in release
                let diff = ((goal as i32) - *pos as i32).abs();
                let diff_lookup = &(diff as usize);
                if lookup_map.contains_key(diff_lookup) {
                    fuel += lookup_map.get(diff_lookup).unwrap();
                } else {
                    let total = ((diff + 1) * diff) / 2;
                    fuel += total;
                    lookup_map.insert(diff as usize, total);
                }
            }
        }
    }

    fuel as u32
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day7/input.txt").unwrap();
    let reader = BufReader::new(file);

    let positions: Vec<usize> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let max = positions.iter().max().unwrap();
    let mut lookup_map: HashMap<usize, i32> = HashMap::new();
    let mut fuel_costs = vec![];
    for i in 0..max + 1 {
        fuel_costs.push(move_to(&positions, i, &variant, &mut lookup_map));
    }

    let min = fuel_costs.iter().min().unwrap();

    println!("Answer - {}", min);
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
