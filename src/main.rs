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

    let mut rules: HashMap<char, (char, u128)> = HashMap::new();

    rules.insert('>', ('<', 25137));
    rules.insert(')', ('(', 3));
    rules.insert(']', ('[', 57));
    rules.insert('}', ('{', 1197));

    rules.insert('(', (')', 1));
    rules.insert('[', (']', 2));
    rules.insert('{', ('}', 3));
    rules.insert('<', ('>', 4));

    let mut scores: Vec<u128> = Vec::new();

    for line in reader.lines() {
        let mut stack: Vec<char> = Vec::new();
        let mut corrupted = false;

        for character in line.unwrap().chars() {
            match character {
                '<' | '(' | '{' | '[' => stack.push(character),
                '>' | ')' | '}' | ']' => {
                    let previous = stack.pop();
                    let (matching, _) = rules.get(&character).unwrap();
                    if previous != Some(*matching) {
                        corrupted = true;
                        break;
                    }
                }
                _ => panic!("unknown character found"),
            }
        }

        if corrupted {
            continue;
        }

        let mut score: u128 = 0;
        for character in stack.iter().rev() {
            let (_, points) = rules.get(&character).unwrap();

            score = score * 5;
            score += *points;
        }
        scores.push(score);
    }

    scores.sort();

    let winning_index = scores.len() / 2;
    println!("{:?}", scores[winning_index]);
}
