#![allow(dead_code)]
#![allow(unused)]

mod helpers;

use anyhow::{Error, Result};
use std::cmp::Ordering::{self, Less};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::num::ParseIntError;
use std::slice::SliceIndex;
use std::str::FromStr;
use std::{fmt, fmt::Debug, fs::File, io::BufRead, io::BufReader};

use itertools::Itertools;

use crate::helpers::print_2d;
use crate::helpers::StringUtils;

const FILENAME: &str = "./inputs/2022/day5/input.txt";

#[derive(Debug)]
struct Stacks {
    pub stacks: Vec<VecDeque<char>>,
}

impl Stacks {
    pub fn new() -> Self {
        Self { stacks: Vec::new() }
    }

    pub fn parse_line(&mut self, line: String) {
        line.chars()
            .chunks(4)
            .into_iter()
            .enumerate()
            .for_each(|(count, stack)| {
                let current_char = stack.skip(1).next().unwrap();
                if current_char.is_digit(10) {
                    return;
                }

                if (self.stacks.len() < count + 1) {
                    self.stacks.push(VecDeque::new());
                }

                let current_stack = self.stacks.get_mut(count).unwrap();
                if (current_char == ' ') {
                    return;
                }
                current_stack.push_front(current_char);
            });
    }

    pub fn pop_many(&mut self, from: usize, amount: &usize) -> VecDeque<char> {
        let initial_stack = self.stacks.get_mut(from).unwrap();
        let final_len = initial_stack.len() - amount;
        let results = initial_stack.split_off(final_len);
        println!("{:?}", results);
        results
    }

    pub fn append_to(&mut self, to: usize, chars: &mut VecDeque<char>) {
        let current_stack = self.stacks.get_mut(to).unwrap();
        for character in chars.iter() {
            current_stack.push_back(*character);
        }
    }
}

impl fmt::Display for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in self.stacks[0].iter() {
            write!(f, "----");
        }
        writeln!(f, "");
        let max = self.stacks.iter().map(|s| s.len()).max().unwrap();

        for row in self.stacks.iter() {
            for col in row.iter() {
                write!(f, " {} |", col);
            }
            writeln!(f, "");
            for _ in row.iter() {
                write!(f, "----");
            }
            writeln!(f, "");
        }

        Ok(())
    }
}

fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let mut lines = BufReader::new(file).lines();
    let mut stacks = Stacks::new();
    while let Some(Ok(line)) = lines.next() {
        if line == "" {
            break;
        }

        stacks.parse_line(line);
    }

    println!("{}", stacks);
    while let Some(Ok(line)) = lines.next() {
        let action: Vec<usize> = line
            .split(" ")
            .filter_map(|part| part.safe_parse::<usize>())
            .collect();

        let amount = action.get(0).unwrap();
        let from = action.get(1).unwrap() - 1;
        let to = action.get(2).unwrap() - 1;
        let mut elements = stacks.pop_many(from, amount);
        println!("");
        println!("Action amount: {}, from: {}, to: {}", amount, from, to);
        println!("");
        stacks.append_to(to, &mut elements);
        println!("{}", stacks);
    }
}
