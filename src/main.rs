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
    part_fn_vec[part_num - 1]();
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
        _ => {
            println!("Day {} not found", day);
        }
    }
}
