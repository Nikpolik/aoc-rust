#![allow(dead_code)]
#![allow(unused)]

mod helpers;

use std::cmp::Ordering::{self, Less};
use std::collections::{HashMap, HashSet};
use std::{fmt, fmt::Debug, fs::File, io::BufRead, io::BufReader};

const FILENAME: &str = "./inputs/day_10/input.txt";

// maybe point arrays where better? You can also destruture and patter match.

fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);
    let mut rules: HashMap<char, (char, u32)> = HashMap::new();
    rules.insert('>', ('<', 25137));
    rules.insert(')', ('(', 3));
    rules.insert(']', ('[', 57));
    rules.insert('}', ('{', 1197));

    let mut score = 0;
    for line in reader.lines() {
        let mut stack: Vec<char> = Vec::new();

        for character in line.unwrap().chars() {
            match character {
                '<' | '(' | '{' | '[' => stack.push(character),
                '>' | ')' | '}' | ']' => {
                    let previous = stack.pop();
                    let (matching, points) = rules.get(&character).unwrap();
                    if previous != Some(*matching) {
                        score += *points;
                        break;
                    }
                }
                _ => {
                    println!("{}", character);
                    panic!("unknown character found");
                }
            }
        }
    }
    println!("{}", score);
}
