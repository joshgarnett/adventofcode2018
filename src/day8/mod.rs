use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::SplitWhitespace;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn parse_node(data: &mut SplitWhitespace) -> Node {
    let children_count: usize = data.next().unwrap().parse::<usize>().expect("not an int");
    let metadata_count: usize = data.next().unwrap().parse::<usize>().expect("not an int");

    Node {
        children: (0..children_count).map(|_| parse_node(data)).collect(),
        metadata: (0..metadata_count).map(|_| data.next().unwrap().parse::<usize>().expect("not an int")).collect(),
    }
}

fn parse_tree(filename: &str) -> Node {
    let file = File::open(filename).expect("file not found");
    for line in BufReader::new(file).lines() {
        let mut data = line.unwrap();
        let mut parts = data.split_whitespace();
        return parse_node(&mut parts);
    }
    unreachable!("failed to parse node");
}

fn sum_metadata(node: &Node) -> usize {
    let mut total: usize = node.metadata.iter().sum();
    for c in &node.children {
        total += sum_metadata(&c);
    }
    total
}

pub fn part1(filename: &str) -> usize {
    let node = parse_tree(filename);
    sum_metadata(&node)
}

fn sum_metadata_part2(node: &Node) -> usize {
    if node.children.len() == 0 {
        node.metadata.iter().sum()
    } else {
        let mut total: usize = 0;
        for m in &node.metadata {
            if *m == 0 || (m - 1) >= node.children.len() {
                continue;
            }
            total += sum_metadata_part2(&node.children[m - 1]);
        }
        total
    }
}

pub fn part2(filename: &str) -> usize {
    let node = parse_tree(filename);
    sum_metadata_part2(&node)
}