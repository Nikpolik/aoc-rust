#![allow(dead_code)]
#![allow(unused)]

mod caves;
mod helpers;

use std::cmp::Ordering::{self, Less};
use std::collections::{HashMap, HashSet};
use std::slice::SliceIndex;
use std::{fmt, fmt::Debug, fs::File, io::BufRead, io::BufReader};

use caves::Caves;
use itertools::Itertools;

use crate::helpers::print_2d;

const FILENAME: &str = "./inputs/day_12/input.txt";

// maybe point arrays where better? You can also destruture and patter match.

fn is_lowercase(value: &String) -> bool {
    value.to_lowercase() == *value
}

fn solve(connections: &HashMap<String, Vec<String>>, path: &mut Vec<String>) {
    let node = path.last().unwrap();
    if node == "end" {
        println!("solved");
        path.pop();
        return;
    }
    if let Some(adj) = connections.get(node) {
        for adj_node in adj {
            if is_lowercase(adj_node) && path.contains(adj_node) {
                continue;
            }
            path.push(adj_node.clone());
            solve(connections, path);
        }
    };
    path.pop();
}

fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let mut reader = BufReader::new(file);
    // there is propably a better way to initialize
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let mut split = line.split('-');
        let first = split.next().unwrap().to_string();
        let second = split.next().unwrap().to_string();
        match connections.get_mut(&first) {
            Some(prev) => prev.push(second.clone()),
            None => {
                let new_connections = vec![second.clone()];
                connections.insert(first.clone(), new_connections);
            }
        };
        match connections.get_mut(&second) {
            Some(prev) => prev.push(first),
            None => {
                let new_connections = vec![first];
                connections.insert(second, new_connections);
            }
        };
    }

    let caves: Caves = Caves::new(&connections);
    let mut path = vec!["start".to_string()];
    let mut visited = HashSet::new();
    visited.insert("start".to_string());
    println!("{}", caves.solve(&mut path, &mut visited, None));
}
