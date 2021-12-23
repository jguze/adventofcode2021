use std::time::Instant;

mod days;
use days::*;

extern crate clap;
use clap::{App, Arg};

fn run_part(part_fn_vec: Vec<fn()>, part: &str) {
    let part_num = part
        .parse::<usize>()
        .expect("Valid part number must be chosen");

    if part_num > part_fn_vec.len() {
        panic!("Part {} doesn't exist", part_num);
    }

    println!("Part {}", part_num);

    let now = Instant::now();

    part_fn_vec[part_num - 1]();
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    let matches = App::new("Advent of Code 2021")
        .version("1.0")
        .author("jguze")
        .arg(
            Arg::with_name("day")
                .short("d")
                .multiple(false)
                .required(true)
                .takes_value(true)
                .help("chooses the day"),
        )
        .arg(
            Arg::with_name("part")
                .short("p")
                .multiple(true)
                .required(true)
                .takes_value(true)
                .default_value("1")
                .help("chooses the part"),
        )
        .get_matches();

    let day = matches.value_of("day").unwrap();
    let part = matches.value_of("part").unwrap();

    print!("Day {} - ", day);
    match day {
        "1" => {
            run_part(vec![day1::part1, day1::part2], part);
        }
        "2" => {
            run_part(vec![day2::part1, day2::part2], part);
        }
        "3" => {
            run_part(vec![day3::part1, day3::part2], part);
        }
        "4" => {
            run_part(vec![day4::part1, day4::part2], part);
        }
        "5" => {
            run_part(vec![day5::part1, day5::part2], part);
        }
        "6" => {
            run_part(vec![day6::part1, day6::part2], part);
        }
        "7" => {
            run_part(vec![day7::part1, day7::part2], part);
        }
        "8" => {
            run_part(vec![day8::part1, day8::part2], part);
        }
        "9" => {
            run_part(vec![day9::part1, day9::part2], part);
        }
        "10" => {
            run_part(vec![day10::part1, day10::part2], part);
        }
        "11" => {
            run_part(vec![day11::part1, day11::part2], part);
        }
        "12" => {
            run_part(vec![day12::part1, day12::part2], part);
        }
        "13" => {
            run_part(vec![day13::part1, day13::part2], part);
        }
        "14" => {
            run_part(vec![day14::part1, day14::part2], part);
        }
        "15" => {
            run_part(vec![day15::part1, day15::part2], part);
        }
        "16" => {
            run_part(vec![day16::part1, day16::part2], part);
        }
        "17" => {
            run_part(vec![day17::part1, day17::part2], part);
        }
        "18" => {
            run_part(vec![day18::part1, day18::part2], part);
        }
        _ => {
            println!("Day {} not found", day);
        }
    }
}
