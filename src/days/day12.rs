use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::rc::Rc;

enum QVariant {
    Part1,
    Part2,
}

const START_ID: &str = "start";
const END_ID: &str = "end";

// Linked lists, graphs, etc are not trivial in Rust
// See:
// 1. https://github.com/nrc/r4cppp/blob/master/graphs/README.md
// 2. https://rust-unofficial.github.io/too-many-lists/index.html
// 3. https://stackoverflow.com/questions/57857832/implement-a-graph-structure-in-rust
type Link = Rc<RefCell<Node>>;

struct Node {
    id: String,
    edges: Vec<Link>,
}

impl Node {
    fn is_start(&self) -> bool {
        self.id == START_ID
    }

    fn is_end(&self) -> bool {
        self.id == END_ID
    }

    fn is_large(&self) -> bool {
        self.id.chars().nth(0).unwrap().is_uppercase()
    }
}

fn find_paths(
    node: &Node,
    cur_path: &mut Vec<String>,
    visited: &mut HashMap<String, u32>,
    paths: &mut Vec<Vec<String>>,
    small_cave_max: bool,
    variant: &QVariant,
) {
    if node.is_end() {
        paths.push(cur_path.clone());
        return;
    }

    let mut small_cave_max = small_cave_max;

    {
        let count = visited.entry(node.id.to_string()).or_insert(0);
        match variant {
            QVariant::Part1 => {
                if *count >= 1 {
                    return;
                }
            }
            QVariant::Part2 => {
                if *count == 2 {
                    return;
                } else if *count >= 1 && small_cave_max {
                    return;
                }
            }
        }

        // Never return to start
        if node.is_start() {
            *count += 2;
        } else if !node.is_large() {
            *count += 1;

            // After visiting a small cave twice, you now can only visit remaining small caves once
            if *count == 2 {
                small_cave_max = true;
            }
        }
    }

    for edge in &node.edges {
        let edge = edge.as_ref().borrow();
        cur_path.push(edge.id.to_string());
        find_paths(&edge, cur_path, visited, paths, small_cave_max, variant);
        cur_path.pop();
    }

    if !node.is_large() {
        // Have to pull this out again to appease the mutability reference,
        // since we pass it into the previous function recursively
        let count = visited.get_mut(&node.id).unwrap();
        *count -= 1;
    }
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day12/input.txt").unwrap();
    let reader = BufReader::new(file);

    // Today I learned about Rc and RefCell very much the hard way
    // See: https://doc.rust-lang.org/std/cell/index.html#introducing-mutability-inside-of-something-immutable
    let shared_node_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));

    for line in reader.lines() {
        let input = line.unwrap();

        let nodes: Vec<String> = input.split('-').map(|s| s.to_string()).collect();
        let node1_id = &nodes[0];
        let node2_id = &nodes[1];

        /*
            This specific part was actually really annoying.
            All I wanted to do was to create the nodes, and add them to a hashmap
            if they didn't exist. That way, when I create edges, I could look for previously-created
            nodes.
            However, this is just not a Rust way. Someone has to own the nodes. Moreover, you can't just
            keep accessing the hashmap and pushing things in constantly with entry.
            To work around this, we use the Rc<RefCell<T>> technique. We make a reference counter to a
            mutable container.
            See:
                1. https://github.com/nrc/r4cppp/blob/master/graphs/README.md
                2. https://rust-unofficial.github.io/too-many-lists/index.html
                3. https://stackoverflow.com/questions/57857832/implement-a-graph-structure-in-rust
        */
        {
            let mut map = shared_node_map.borrow_mut();
            map.entry(node1_id.to_string())
                .or_insert(Rc::new(RefCell::new(Node {
                    id: node1_id.to_string(),
                    edges: vec![],
                })));

            map.entry(node2_id.to_string())
                .or_insert(Rc::new(RefCell::new(Node {
                    id: node2_id.to_string(),
                    edges: vec![],
                })));
        }

        let map = shared_node_map.borrow();
        let node1_rc = map.get(node1_id).unwrap();
        let node2_rc = map.get(node2_id).unwrap();

        let mut node = node1_rc.as_ref().borrow_mut();
        node.edges.push(Rc::clone(node2_rc));

        let mut node2 = node2_rc.as_ref().borrow_mut();
        node2.edges.push(Rc::clone(node1_rc))
    }

    let node_map = shared_node_map.borrow();
    let start_node = node_map.get(START_ID).unwrap().as_ref().borrow();

    let mut paths = vec![];
    let mut cur_path = vec![];
    cur_path.push(start_node.id.to_string());
    find_paths(
        &start_node,
        &mut cur_path,
        &mut HashMap::new(),
        &mut paths,
        false,
        &variant,
    );

    println!("Answer - {}", paths.len());
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
