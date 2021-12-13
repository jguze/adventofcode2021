use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

const DOT: char = '■';
const EMPTY: char = '.';

enum QVariant {
    Part1,
    Part2,
}

#[derive(Debug, Hash, Copy, Clone)]
struct Point(u32, u32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

#[derive(Debug)]
enum FoldDir {
    X,
    Y,
}

#[derive(Debug)]
struct Board {
    grid: HashSet<Point>,
    row_map: HashMap<u32, HashSet<Point>>,
    col_map: HashMap<u32, HashSet<Point>>,
    max_row: u32,
    max_col: u32,
}

impl Board {
    fn new() -> Board {
        Board {
            row_map: HashMap::new(),
            col_map: HashMap::new(),
            grid: HashSet::new(),
            max_row: 0,
            max_col: 0,
        }
    }

    fn add_point(&mut self, point: Point) {
        if point.0 > self.max_col {
            self.max_col = point.0;
        }

        if point.1 > self.max_row {
            self.max_row = point.1;
        }

        self.grid.insert(point);
        let col_set = self.col_map.entry(point.0).or_insert(HashSet::new());
        col_set.insert(point);

        let row_set = self.row_map.entry(point.1).or_insert(HashSet::new());
        row_set.insert(point);
    }

    fn remove_point(&mut self, point: Point) {
        self.grid.remove(&point);

        if self.col_map.contains_key(&point.0) {
            let col_set = self.col_map.get_mut(&point.0).unwrap();
            col_set.remove(&point);

            if col_set.len() == 0 {
                self.col_map.remove(&point.0);
            }
        }

        if self.row_map.contains_key(&point.1) {
            let row_set = self.row_map.get_mut(&point.1).unwrap();
            row_set.remove(&point);

            if row_set.len() == 0 {
                self.row_map.remove(&point.1);
            }
        }
    }
}

/*
   Thoughts -
   Sparse array (Hashmap of points)

   Hashmap of row -> Vector of points in row
   Hashmap of col -> Vectors of points in col

   0 - 20

   fold on 12

   19 -> 12 - 19 + 12

   0 - 14

   7 - (14 - 7)

   n- 1 -> 0
   n - 2 -> 1
   n - 3 -> 2
   etc
*/

fn parse_input(reader: BufReader<File>) -> (Board, Vec<(FoldDir, u32)>) {
    lazy_static! {
        static ref FOLD_RE: Regex = Regex::new(r"^fold along ([a-z])=([0-9]+)$").unwrap();
    }

    let mut board = Board::new();
    let mut folds = vec![];

    let mut line_iter = reader.lines().peekable();
    while let Some(line_opt) = line_iter.next() {
        let line = line_opt.unwrap();
        if line.len() == 0 {
            break;
        }

        let coords = line.split(",").collect::<Vec<&str>>();
        let point = Point(
            coords[0].parse::<u32>().unwrap(),
            coords[1].parse::<u32>().unwrap(),
        );

        board.add_point(point);
    }

    while let Some(line_opt) = line_iter.next() {
        let line = line_opt.unwrap();
        let captures = FOLD_RE.captures_iter(line.as_str());

        for cap in captures {
            let fold_dir = match &cap[1] {
                "x" => FoldDir::X,
                "y" => FoldDir::Y,
                _ => panic!("Unexpected fold direction"),
            };

            folds.push((fold_dir, cap[2].parse::<u32>().unwrap()));
        }
    }

    (board, folds)
}

fn fold_board(board: &mut Board, folds: &Vec<(FoldDir, u32)>, variant: &QVariant) {
    for fold in folds {
        let mut points_to_add: Vec<Point> = vec![];
        let mut points_to_remove: Vec<Point> = vec![];
        match fold.0 {
            FoldDir::X => {
                for x in (fold.1 + 1)..(board.max_col + 1) {
                    let maybe_col = board.col_map.get(&x);
                    if maybe_col.is_none() {
                        continue;
                    }

                    let col = maybe_col.unwrap();
                    for point in col {
                        points_to_add.push(Point(2 * fold.1 - point.0, point.1));
                        points_to_remove.push(point.clone());
                    }
                }
            }
            FoldDir::Y => {
                for y in (fold.1 + 1)..(board.max_row + 1) {
                    let maybe_row = board.row_map.get(&y);
                    if maybe_row.is_none() {
                        continue;
                    }

                    let row = maybe_row.unwrap();
                    for point in row {
                        points_to_add.push(Point(point.0, 2 * fold.1 - point.1));
                        points_to_remove.push(point.clone());
                    }
                }
            }
        }

        // Doing this outside the fold in the match because borrow checker gets mad
        for point in points_to_add {
            board.add_point(point);
        }

        for point in points_to_remove {
            board.remove_point(point);
        }

        if matches!(variant, QVariant::Part1) {
            break;
        }
    }
}
fn print_grid(board: &Board) {
    let mut max_col = 0;
    let mut max_row = 0;
    board.grid.iter().for_each(|p| {
        if p.0 > max_col {
            max_col = p.0;
        }

        if p.1 > max_row {
            max_row = p.1;
        }
    });
    for j in 0..max_row + 1 {
        for i in 0..max_col + 1 {
            if board.grid.contains(&Point(i, j)) {
                print!("{} ", DOT);
            } else {
                print!("{} ", EMPTY);
            }
        }
        println!("");
    }
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day13/input.txt").unwrap();
    let reader = BufReader::new(file);

    let (mut board, folds) = parse_input(reader);

    fold_board(&mut board, &folds, &variant);

    match variant {
        QVariant::Part1 => {
            println!("Answer - {}", board.grid.len());
        }
        QVariant::Part2 => {
            print_grid(&board);
            println!("Answer - {}", board.grid.len());
        }
    }
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
