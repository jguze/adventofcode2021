use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

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
struct TargetArea {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

fn parse_input(input: &str) -> TargetArea {
    lazy_static! {
        static ref PARSER: Regex = Regex::new(
            r"^target area: x=([-]?[0-9]+)..([-]?[0-9]+), y=([-]?[0-9]+)..([-]?[0-9]+)$"
        )
        .unwrap();
    }

    let captures = PARSER.captures_iter(input);

    for cap in captures {
        let left = cap[1].parse::<i32>().unwrap();
        let right = cap[2].parse::<i32>().unwrap();
        let bottom = cap[3].parse::<i32>().unwrap();
        let top = cap[4].parse::<i32>().unwrap();

        return TargetArea {
            left,
            right,
            bottom,
            top,
        };
    }

    panic!("Failed to read target area");
}

fn max_y_for_y_vel(start_pos: &Point, y_vel: i32) -> i32 {
    return start_pos.1 + (y_vel) * (y_vel + 1) / 2;
}

fn will_yvel_hit_at_time(start_pos: &Point, target_area: &TargetArea, y_vel: i32) -> Vec<u32> {
    let mut t = 0;

    let mut times = vec![];
    let mut current_y = start_pos.0;
    let mut current_y_vel = y_vel;
    let mut hit_top = false;
    let mut current_max: Option<i32> = None;
    while current_y >= target_area.bottom || !hit_top {
        if current_max.is_none() {
            current_max = Some(current_y);
        } else {
            if current_y > current_max.unwrap() {
                current_max = Some(current_y);
            } else if current_y < current_max.unwrap() {
                hit_top = true;
            }
        }

        if current_y >= target_area.bottom && current_y <= target_area.top {
            times.push(t);
        }

        current_y += current_y_vel;
        current_y_vel -= 1;
        t += 1;
    }

    times
}

fn generate_yvels_in_target(start_pos: &Point, target_area: &TargetArea) -> Vec<(i32, u32)> {
    let mut y_vel = target_area.bottom;
    let mut misses = 0;

    let mut output = vec![];
    // I'm not smart enough to know when we can no longer hit, so let's just assume after
    // 1000 misses that we'll never hit it
    while misses < 1000 {
        let times = will_yvel_hit_at_time(start_pos, target_area, y_vel);
        if times.len() == 0 {
            misses += 1;
        } else {
            for t in times {
                output.push((y_vel, t));
            }
        }

        y_vel += 1;
    }

    output
}

fn will_xvel_hit_at_time(start_pos: &Point, target_area: &TargetArea, x_vel: i32, t: u32) -> bool {
    let mut current_x_vel = x_vel;

    let mut current_x = start_pos.0;
    for _ in 0..t {
        if current_x_vel == 0 {
            break;
        }
        current_x += current_x_vel;
        current_x_vel -= 1;
    }

    current_x <= target_area.right && current_x >= target_area.left
}

fn generate_trajectories_for_y_vels(
    start_point: &Point,
    target_area: &TargetArea,
    y_vels: &Vec<(i32, u32)>,
) -> HashSet<Point> {
    let mut map = HashSet::new();

    for (y_vel, t) in y_vels {
        let mut x_vel = 1;
        while x_vel <= target_area.right {
            let result = will_xvel_hit_at_time(start_point, target_area, x_vel, *t);
            if result {
                map.insert(Point(x_vel, *y_vel));
            }

            x_vel += 1;
        }
    }

    map
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day17/input.txt").unwrap();
    let reader = BufReader::new(file);

    let target_area = parse_input(&reader.lines().next().unwrap().unwrap());

    let start_pos = Point(0, 0);
    let y_vels = generate_yvels_in_target(&start_pos, &target_area);

    // Given a set of y velocities that hit at time = t, we can figure out x velos that hit it
    let point_map = generate_trajectories_for_y_vels(&start_pos, &target_area, &y_vels);

    match variant {
        QVariant::Part1 => {
            println!(
                "Answer - {}",
                y_vels
                    .iter()
                    .map(|(y, _)| max_y_for_y_vel(&start_pos, *y))
                    .max()
                    .unwrap()
            );
        }
        QVariant::Part2 => {
            println!("Answer - {}", point_map.len());
        }
    }
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
