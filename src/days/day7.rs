use std::fs::File;
use std::io::{prelude::*, BufReader};

enum QVariant {
    Part1,
    Part2,
}

fn move_to(positions: &Vec<usize>, goal: usize, variant: &QVariant) -> u32 {
    let mut fuel = 0;
    for pos in positions {
        match variant {
            QVariant::Part1 => fuel += (goal as i32 - *pos as i32).abs(),
            QVariant::Part2 => fuel += (0..(goal as i32 - *pos as i32).abs() + 1).sum::<i32>(),
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
    let mut fuel_costs = vec![];
    for i in 0..max + 1 {
        fuel_costs.push(move_to(&positions, i, &variant));
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
