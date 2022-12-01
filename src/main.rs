#![allow(dead_code)]
#![allow(unused)]

mod helpers;

use std::cmp::Ordering::{self, Less};
use std::collections::{HashMap, HashSet};
use std::slice::SliceIndex;
use std::{fmt, fmt::Debug, fs::File, io::BufRead, io::BufReader};

use itertools::Itertools;

use crate::helpers::print_2d;
use crate::helpers::StringUtils;

const FILENAME: &str = "./inputs/2022/day_1/input.txt";

fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);
    let mut top_elves: Vec<u64> = Vec::new();
    let mut current_calories: u64 = 0;
    for line in reader.lines().map(|line| line.expect("invalid line")) {
        // .filter_map(|line| line.expect("Read invalid line").safe_parse::<u64>())
        if (line == "") {
            top_elves.push(current_calories);
            top_elves.sort_by(|a, b| b.cmp(a));
            if (top_elves.len() > 3) {
                top_elves.pop();
            }
            println!("{:?}", top_elves);
            current_calories = 0;
            continue;
        }

        match line.parse::<u64>() {
            Ok(v) => current_calories += v,
            Err(_) => {
                println!("Expected number but could not parse : {}", line);
                panic!();
            }
        };
    }
    println!("{}", top_elves.iter().sum::<u64>())
}
