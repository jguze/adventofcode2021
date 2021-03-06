use std::cell::RefCell;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::rc::Rc;

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

enum SnailValue {
    Pair(SnailNum),
    Regular(u32),
}

struct SnailNum {
    parent: RefSnailNum,
    left: RefSnailValue,
}

type RefSnailNum = Rc<RefCell<SnailNum>>;
type RefSnailValue = Rc<RefCell<SnailValue>>;

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day/input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let input = line.unwrap();
    }

    match variant {
        QVariant::Part1 => {
            println!("Answer - {}", 1);
        }
        QVariant::Part2 => {
            println!("Answer - {}", 2);
        }
    }
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
