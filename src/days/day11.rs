use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

enum QVariant {
    Part1,
    Part2,
}

#[derive(Debug, Hash, Copy, Clone)]
struct Point(usize, usize);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

const FLASH_THRESHOLD: u32 = 9;

fn propogate(
    point: &Point,
    jellyfish: &mut Vec<Vec<u32>>,
    to_flash: &mut Vec<Point>,
    flashed: &mut HashSet<Point>,
) {
    let row_len = jellyfish.len();
    let col_len = jellyfish[0].len();

    let directions: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    if flashed.contains(point) {
        return;
    }

    flashed.insert(*point);

    for dir in directions {
        if dir.0 < 0 && point.0 == 0 {
            continue;
        }

        if dir.1 < 0 && point.1 == 0 {
            continue;
        }

        if dir.0 > 0 && point.0 == row_len - 1 {
            continue;
        }

        if dir.1 > 0 && point.1 == col_len - 1 {
            continue;
        }

        let new_point = Point(
            (point.0 as i32 + dir.0) as usize,
            (point.1 as i32 + dir.1) as usize,
        );
        if flashed.contains(&new_point) {
            continue;
        }

        jellyfish[new_point.0][new_point.1] += 1;

        if jellyfish[new_point.0][new_point.1] > FLASH_THRESHOLD {
            to_flash.push(new_point);
        }
    }
}

fn step(jellyfish: &mut Vec<Vec<u32>>) -> u32 {
    let row_len = jellyfish.len();
    let col_len = jellyfish[0].len();

    let mut to_flash: Vec<Point> = vec![];
    let mut flashed: HashSet<Point> = HashSet::new();

    for row in 0..row_len {
        for col in 0..col_len {
            jellyfish[row][col] += 1;
            if jellyfish[row][col] > FLASH_THRESHOLD {
                to_flash.push(Point(row, col));
            }
        }
    }

    while let Some(point) = to_flash.pop() {
        propogate(&point, jellyfish, &mut to_flash, &mut flashed);
    }

    for point in &flashed {
        jellyfish[point.0][point.1] = 0;
    }

    flashed.len() as u32
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day11/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut jellyfish: Vec<Vec<u32>> = vec![];
    for line in reader.lines() {
        let input = line.unwrap();
        jellyfish.push(input.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let total_jellyfish: u32 = (jellyfish.len() * jellyfish[0].len()) as u32;

    let mut flash_count = 0;
    let mut step_count = 0;

    loop {
        let current_flashes = step(&mut jellyfish);
        flash_count += current_flashes;
        step_count += 1;

        if matches!(variant, QVariant::Part1) && step_count == 100 {
            break;
        }

        if matches!(variant, QVariant::Part2) && current_flashes == total_jellyfish {
            break;
        }
    }

    /*
       1. 2x for loop - increase by 1. If anything > 9, put in to_flash queue
       2. For each pair in to_flash queue
          a. Add to flashed set.
          b. Increase all neighbours by 1. Add to flash queue if > 9
       3. For each pair in flashed set - set to 0
    */

    match variant {
        QVariant::Part1 => {
            println!("Answer - {}", flash_count);
        }
        QVariant::Part2 => {
            println!("Answer - {}", step_count);
        }
    }
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
