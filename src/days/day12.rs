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
    is_start: bool,
    is_end: bool,
    is_large: bool,
}

impl Node {
    fn new(id: String) -> Node {
        Node {
            edges: vec![],
            is_start: id == START_ID,
            is_end: id == END_ID,
            is_large: id.chars().nth(0).unwrap().is_uppercase(),
            id,
        }
    }
}

fn find_paths(
    node: &Node,
    visited: &mut HashMap<String, u32>,
    small_cave_max: bool,
    variant: &QVariant,
) -> u32 {
    if node.is_end {
        return 1;
    }

    let mut small_cave_max = small_cave_max;

    {
        let count = visited.entry(node.id.to_string()).or_insert(0);
        match variant {
            QVariant::Part1 => {
                if *count >= 1 {
                    return 0;
                }
            }
            QVariant::Part2 => {
                if *count == 2 {
                    return 0;
                } else if *count >= 1 && small_cave_max {
                    return 0;
                }
            }
        }

        // Never return to start
        if node.is_start {
            *count += 2;
        } else if !node.is_large {
            *count += 1;

            // After visiting a small cave twice, you now can only visit remaining small caves once
            if *count == 2 {
                small_cave_max = true;
            }
        }
    }

    let mut total_paths = 0;
    for edge in &node.edges {
        let edge = edge.as_ref().borrow();
        total_paths += find_paths(&edge, visited, small_cave_max, variant);
    }

    if !node.is_large {
        // Have to pull this out again to appease the mutability reference,
        // since we pass it into the previous function recursively
        let count = visited.get_mut(&node.id).unwrap();
        *count -= 1;
    }

    total_paths
}

fn run_problem(variant: QVariant) {
    let file = File::open("inputs/day12/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut node_map: HashMap<_, _> = HashMap::new();

    for line in reader.lines() {
        let input = line.unwrap();

        let nodes: Vec<String> = input.split('-').map(|s| s.to_string()).collect();
        let node1_id = &nodes[0];
        let node2_id = &nodes[1];

        /*
            Today I learned about Rc and RefCell very much the hard way
            See: https://doc.rust-lang.org/std/cell/index.html#introducing-mutability-inside-of-something-immutable
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
        node_map
            .entry(node1_id.to_string())
            .or_insert(Rc::new(RefCell::new(Node::new(node1_id.to_string()))));

        node_map
            .entry(node2_id.to_string())
            .or_insert(Rc::new(RefCell::new(Node::new(node2_id.to_string()))));

        let node1_rc = node_map.get(node1_id).unwrap();
        let node2_rc = node_map.get(node2_id).unwrap();

        let mut node = node1_rc.as_ref().borrow_mut();
        node.edges.push(Rc::clone(node2_rc));

        let mut node2 = node2_rc.as_ref().borrow_mut();
        node2.edges.push(Rc::clone(node1_rc))
    }

    let start_node = node_map.get(START_ID).unwrap().as_ref().borrow();

    let paths = find_paths(&start_node, &mut HashMap::new(), false, &variant);

    println!("Answer - {}", paths);
}

pub fn part1() {
    run_problem(QVariant::Part1);
}

pub fn part2() {
    run_problem(QVariant::Part2);
}
