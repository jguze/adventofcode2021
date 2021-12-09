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

const IMPASSABLE_NUM: u32 = 9;

// Simple DFS. We have the advantage of knowing that basins will NEVER touch each other
fn determine_basin(point: Point, height_map: &Vec<Vec<u32>>, visited: &mut HashSet<Point>) -> u32 {
    if visited.contains(&point) {
        return 0;
    }

    visited.insert(point);

    let current_val = height_map[point.0][point.1];
    if current_val == IMPASSABLE_NUM {
        return 0;
    }

    let mut size = 0;

    let row_len = height_map.len();
    let col_len = height_map[0].len();

    if point.0 > 0 && height_map[point.0 - 1][point.1] >= current_val {
        size += determine_basin(Point(point.0 - 1, point.1), &height_map, visited);
    }

    if point.1 > 0 && height_map[point.0][point.1 - 1] >= current_val {
        size += determine_basin(Point(point.0, point.1 - 1), &height_map, visited);
    }

    if point.0 < row_len - 1 && height_map[point.0 + 1][point.1] >= current_val {
        size += determine_basin(Point(point.0 + 1, point.1), &height_map, visited);
    }

    if point.1 < col_len - 1 && height_map[point.0][point.1 + 1] >= current_val {
        size += determine_basin(Point(point.0, point.1 + 1), &height_map, visited);
    }

    size + 1
}

fn find_basins(lowest_points: &Vec<Point>, height_map: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut basins = vec![];

    let mut visited = HashSet::new();
    for p in lowest_points {
        basins.push(determine_basin(*p, &height_map, &mut visited));
    }

    basins
}

fn is_lowest(i: usize, j: usize, height_map: &Vec<Vec<u32>>) -> bool {
    let row_len = height_map.len();
    let col_len = height_map[0].len();

    let current_val = height_map[i][j];

    if i > 0 && height_map[i - 1][j] <= current_val {
        return false;
    }

    if j > 0 && height_map[i][j - 1] <= current_val {
        return false;
    }

    if i < row_len - 1 && height_map[i + 1][j] <= current_val {
        return false;
    }

    if j < col_len - 1 && height_map[i][j + 1] <= current_val {
        return false;
    }

    true
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day9/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut height_map: Vec<Vec<u32>> = vec![];
    for line in reader.lines() {
        let input = line.unwrap();
        let mut row = vec![];
        for i in input.chars().map(|c| c.to_digit(10).unwrap()) {
            row.push(i);
        }

        height_map.push(row);
    }

    let row_len = height_map.len();
    let col_len = height_map[0].len();

    let mut lowest_points: Vec<Point> = vec![];

    let mut risk = 0;
    for i in 0..row_len {
        for j in 0..col_len {
            if is_lowest(i, j, &height_map) {
                risk += height_map[i][j] + 1;
                lowest_points.push(Point(i, j));
            }
        }
    }

    let mut basins = find_basins(&lowest_points, &height_map);
    basins.sort_by(|a, b| b.cmp(a));

    let mut top_basin_size = 1;
    for basin in &basins[0..3] {
        top_basin_size *= basin;
    }

    match variant {
        QVariant::Part1 => {
            println!("Answer - {}", risk);
        }
        QVariant::Part2 => {
            println!("Answer - {}", top_basin_size);
        }
    }
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
