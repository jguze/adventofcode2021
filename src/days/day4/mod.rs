use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const BINGO_BOARD_SIZE: usize = 5;

#[derive(Debug)]
struct BingoBoard {
    board_state: [[bool; BINGO_BOARD_SIZE]; BINGO_BOARD_SIZE],

    // Easy lookup to go from bingo entry to point on the board
    num_state_map: HashMap<u32, (usize, usize)>,
}

impl Default for BingoBoard {
    fn default() -> BingoBoard {
        BingoBoard {
            board_state: [[false; BINGO_BOARD_SIZE]; BINGO_BOARD_SIZE],
            num_state_map: HashMap::new(),
        }
    }
}

impl BingoBoard {
    fn try_mark_entry(&mut self, entry: u32) -> bool {
        if self.num_state_map.contains_key(&entry) {
            let point = self.num_state_map.get(&entry).unwrap();
            self.board_state[point.0][point.1] = true;
            true
        } else {
            false
        }
    }

    fn add_entry(&mut self, row: usize, col: usize, entry: u32) {
        self.num_state_map.insert(entry, (row, col));
    }

    /**
     * To save a little time, we will only look at the row and column
     * to which that entry belongs
     */
    fn is_entry_winner(&self, entry: u32) -> bool {
        if !self.num_state_map.contains_key(&entry) {
            return false;
        }

        let point = self.num_state_map.get(&entry).unwrap();

        let mut row_all_true = true;
        // iterate row. col is fixed
        for row in 0..BINGO_BOARD_SIZE {
            if !self.board_state[row][point.1] {
                row_all_true = false;
                break;
            }
        }

        if row_all_true {
            return true;
        }

        let mut col_all_true = true;
        // iterate col. row is fixed
        for col in 0..BINGO_BOARD_SIZE {
            if !self.board_state[point.0][col] {
                col_all_true = false;
                break;
            }
        }

        if col_all_true {
            return true;
        }

        false
    }

    fn count_unmarked(&self) -> u32 {
        let mut sum = 0;
        for (entry, point) in &self.num_state_map {
            if !self.board_state[point.0][point.1] {
                sum += entry;
            }
        }

        sum
    }
}

fn parse_input(reader: BufReader<File>) -> (Vec<u32>, Vec<BingoBoard>) {
    let mut line_iter = reader.lines().peekable();

    let line = line_iter.next().unwrap().unwrap();
    let bingo_nums = line
        .split(',')
        .map(|token| token.parse::<u32>().unwrap())
        .collect();

    // Skip the newline
    line_iter.next();

    // Iterate over board input. Boards are separated by a newline
    let mut boards: Vec<BingoBoard> = vec![];
    while line_iter.peek().is_some() {
        let mut board = BingoBoard {
            ..Default::default()
        };
        let mut row = 0;
        let mut col = 0;
        while let Some(line_opt) = line_iter.next() {
            let line = line_opt.unwrap();
            if line.len() == 0 {
                break;
            }

            let board_nums: Vec<u32> = line
                .split_whitespace()
                .map(|token| token.parse::<u32>().unwrap())
                .collect();
            for num in board_nums {
                board.add_entry(row, col, num);
                col += 1;
            }

            row += 1;
            col = 0;
        }

        boards.push(board);
    }

    (bingo_nums, boards)
}

enum BingoVariant {
    Part1,
    Part2,
}

fn run_bingo(variant: BingoVariant) {
    let file = File::open("inputs/day4/input.txt").unwrap();
    let reader = BufReader::new(file);

    let (bingo_nums, mut boards) = parse_input(reader);

    let mut unmarked_count: Option<u32> = None;
    let mut last_entry: Option<u32> = None;

    let mut winners: Vec<usize> = vec![];

    let num_boards = boards.len();

    'outer: for num in &bingo_nums {
        for (i, board) in boards.iter_mut().enumerate() {
            if !winners.contains(&i) {
                board.try_mark_entry(*num);
                if board.is_entry_winner(*num) {
                    winners.push(i);
                    // Part 1 and 2 differ only by choosing either the first winning board, or last one
                    let found_solution = match &variant {
                        BingoVariant::Part1 => true,
                        BingoVariant::Part2 => winners.len() == num_boards,
                    };

                    if found_solution {
                        unmarked_count = Some(board.count_unmarked());
                        last_entry = Some(*num);
                        break 'outer;
                    }
                }
            }
        }
    }

    if unmarked_count.is_none() {
        println!("No bingo boards won with the current input");
    } else {
        println!(
            "Last number called - {}, unmarked count - {}",
            last_entry.unwrap(),
            unmarked_count.unwrap()
        );
        println!("Answer - {}", last_entry.unwrap() * unmarked_count.unwrap());
    }
}

pub fn part1() {
    run_bingo(BingoVariant::Part1);
}

pub fn part2() {
    run_bingo(BingoVariant::Part2);
}
