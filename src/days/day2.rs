use std::fs::File;
use std::io::{prelude::*, BufReader};

enum Direction {
    Up(u32),
    Down(u32),
    Forward(u32),
}

fn parse_direction(dir_text: &str) -> Direction {
    let tokens: Vec<&str> = dir_text.split(" ").collect();

    let distance = tokens[1].parse::<u32>().unwrap();
    match tokens[0] {
        "forward" => Direction::Forward(distance),
        "up" => Direction::Up(distance),
        "down" => Direction::Down(distance),
        _ => panic!("Bad input"),
    }
}

pub fn part1() {
    let file = File::open("inputs/day2/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;

    for line in reader.lines() {
        let text = line.unwrap();

        let direction = parse_direction(&text);
        match direction {
            Direction::Forward(d) => {
                horizontal += d;
            }
            Direction::Down(d) => {
                depth += d;
            }
            Direction::Up(d) => {
                depth -= d;
            }
        }
    }

    println!("Horizontal {}, Depth {}", horizontal, depth);
    println!("Answer - {}", horizontal * depth);
}

pub fn part2() {
    let file = File::open("inputs/day2/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let text = line.unwrap();

        let direction = parse_direction(&text);
        match direction {
            Direction::Forward(d) => {
                horizontal += d;
                depth += aim * d;
            }
            Direction::Down(d) => {
                aim += d;
            }
            Direction::Up(d) => {
                aim -= d;
            }
        }
    }

    println!("Horizontal {}, Depth {}, Aim {}", horizontal, depth, aim);
    println!("Answer - {}", horizontal * depth);
}
