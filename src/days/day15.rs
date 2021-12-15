use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
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

#[derive(Debug, Copy, Clone)]
struct Node {
    weight: u32,
    point: Point,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.point.0 == other.point.0 && self.point.1 == other.point.1
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}

// Forces a min-heap by flipping the cmp
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.weight.cmp(&self.weight))
    }
}

fn create_node(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> Node {
    let x_len = grid.len();
    let y_len = grid[0].len();

    let mut x_shift: u32 = 0;
    let weight_x = if x > x_len - 1 {
        x_shift = x as u32 / x_len as u32;
        x % x_len
    } else {
        x
    };

    let mut y_shift: u32 = 0;
    let weight_y = if y > y_len - 1 {
        y_shift = y as u32 / y_len as u32;
        y % y_len
    } else {
        y
    };

    let weight_rotations = (grid[weight_x][weight_y] + x_shift + y_shift) / 10;
    let weight = (grid[weight_x][weight_y] + x_shift + y_shift) % 10;

    Node {
        weight: weight + weight_rotations,
        point: Point(x, y),
    }
}

fn next_node_pos_valid(point: &Point, dir: (i32, i32), grid_len: (usize, usize)) -> bool {
    if point.0 == 0 && dir.0 < 0 {
        return false;
    }

    if point.1 == 0 && dir.1 < 0 {
        return false;
    }

    if point.0 == grid_len.0 - 1 && dir.0 > 0 {
        return false;
    }

    if point.1 == grid_len.1 - 1 && dir.1 > 0 {
        return false;
    }

    true
}

/*
    This is just Dijkstra's algorithm
*/
fn find_shortest_path_risk(
    grid: &Vec<Vec<u32>>,
    start: Node,
    end: Node,
    visited: &mut HashSet<Node>,
) -> Node {
    let mut p_queue = BinaryHeap::new();
    p_queue.push(start);

    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    let bounds = (end.point.0 + 1, end.point.1 + 1);

    while let Some(node) = p_queue.pop() {
        if node == end {
            return node;
        }

        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        // Get adjacent
        for dir in directions {
            if !next_node_pos_valid(&node.point, dir, bounds) {
                continue;
            }

            let mut new_node = create_node(
                (node.point.0 as i32 + dir.0) as usize,
                (node.point.1 as i32 + dir.1) as usize,
                &grid,
            );

            new_node.weight += node.weight;

            p_queue.push(new_node);
        }
    }

    panic!("Could not find path to end of grid");
}

fn print_grid_visited(grid: &Vec<Vec<u32>>, visited: &HashSet<Node>, end: Node) {
    for i in 0..end.point.0 {
        for j in 0..end.point.1 {
            let node = create_node(i, j, &grid);
            if visited.contains(&node) {
                print!("{}", node.weight);
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

/**
 * Builds a hashset of the points that lead to the
 * shortest path
 */
fn build_shortest_path_set(
    visited: &HashSet<Node>,
    end: Node,
    start: Node,
    grid: &Vec<Vec<u32>>,
) -> HashSet<Node> {
    let mut shortest_path_set = HashSet::new();

    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    let bounds = (end.point.0 + 1, end.point.1 + 1);

    let mut current_node = end;
    'outer: while current_node != start {
        shortest_path_set.insert(current_node);

        let mut adj_nodes = vec![];
        // Get adjacent
        for dir in directions {
            if !next_node_pos_valid(&current_node.point, dir, bounds) {
                continue;
            }

            let new_node = create_node(
                (current_node.point.0 as i32 + dir.0) as usize,
                (current_node.point.1 as i32 + dir.1) as usize,
                &grid,
            );

            if new_node == start {
                break 'outer;
            }

            if shortest_path_set.contains(&new_node) {
                continue;
            }

            if visited.contains(&new_node) {
                adj_nodes.push(visited.get(&new_node).unwrap());
            }
        }

        let mut min_node: Option<Node> = None;
        for node in adj_nodes {
            if min_node.is_none() {
                min_node = Some(*node);
            } else if node.weight < min_node.unwrap().weight {
                min_node = Some(*node);
            }
        }

        current_node = min_node.unwrap();
    }

    shortest_path_set
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day15/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<u32>> = vec![];

    for line in reader.lines() {
        grid.push(
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect(),
        );
    }

    let start = Node {
        point: Point(0, 0),
        weight: 0,
    };

    let end = match variant {
        QVariant::Part1 => create_node(grid.len() - 1, grid[0].len() - 1, &grid),
        QVariant::Part2 => create_node(grid.len() * 5 - 1, grid[0].len() * 5 - 1, &grid),
    };

    let mut visited = HashSet::new();
    let node = find_shortest_path_risk(&grid, start, end, &mut visited);

    let shortest_path_set = build_shortest_path_set(&visited, end, start, &grid);

    print_grid_visited(&grid, &shortest_path_set, end);

    println!("Answer - {}", node.weight);
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
