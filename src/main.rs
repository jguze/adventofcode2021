mod days;
use days::day1;
use days::day2;

extern crate clap;
use clap::{App, Arg};

fn run_part(part1_fn: fn(), part2_fn: fn(), part: &str) {
    match part {
        "1" => {
            println!("Part 1");
            part1_fn();
        }
        "2" => {
            println!("Part 2");
            part2_fn();
        }
        _ => {
            panic!("Part must be chosen");
        }
    }
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
            run_part(day1::part1, day1::part2, part);
        }
        "2" => {
            run_part(day2::part1, day2::part2, part);
        }
        _ => {
            println!("Day {} not found", day);
        }
    }
}
