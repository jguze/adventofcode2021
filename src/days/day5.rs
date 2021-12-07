use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

enum QVariant {
    Part1,
    Part2,
}

#[derive(Debug, Hash, Copy, Clone)]
struct Point(i32, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.end.1 != self.start.1 && self.end.0 != self.start.0
    }

    /**
     * Get the smallest integer unit for the slope.
     * For this problem, it will be some combination of (0,1), (1,0), or (1,1),
     * with different negative signs for direction
     */
    fn slope_unit(&self) -> Point {
        // Since all slopes are either straight, or diagonal, we can cheat a little
        // when normalizing
        let x_diff = self.end.0 - self.start.0;
        let x = if x_diff == 0 {
            0
        } else if x_diff < 0 {
            -1
        } else {
            1
        };
        let y_diff = self.end.1 - self.start.1;
        let y = if y_diff == 0 {
            0
        } else if y_diff < 0 {
            -1
        } else {
            1
        };

        Point(x, y)
    }
}

fn convert_to_point(x: &str, y: &str) -> Point {
    Point(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
}

fn fill_board(lines: &Vec<Line>) -> HashMap<Point, u32> {
    let mut map = HashMap::new();
    for line in lines {
        let unit = line.slope_unit();

        let mut cur_point = line.start;
        while cur_point != line.end {
            let count = map.entry(cur_point).or_insert(0);
            *count += 1;
            cur_point.0 += unit.0;
            cur_point.1 += unit.1;
        }
        // Gotta count the last one
        let count = map.entry(cur_point).or_insert(0);
        *count += 1;
    }

    map
}

fn parse_input(reader: BufReader<File>) -> Vec<Line> {
    // Parse regex quickly without affecting compilation:
    // https://docs.rs/regex/latest/regex/#example-avoid-compiling-the-same-regex-in-a-loop
    lazy_static! {
        static ref INPUT_RE: Regex =
            Regex::new(r"^([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)$").unwrap();
    }

    let mut lines = vec![];

    for line in reader.lines() {
        let text = line.unwrap();
        let captures = INPUT_RE.captures_iter(text.as_str());

        for cap in captures {
            lines.push(Line {
                start: convert_to_point(&cap[1], &cap[2]),
                end: convert_to_point(&cap[3], &cap[4]),
            });
        }
    }

    lines
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day5/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = parse_input(reader);

    if matches!(variant, QVariant::Part1) {
        lines.retain(|l| !l.is_diagonal());
    }

    // Fill a hashmap of points. Then, filter to just the values >= 2
    let map = fill_board(&lines);
    let count = map.values().filter(|v| **v >= 2).count();

    println!("Answer - {}", count);
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
