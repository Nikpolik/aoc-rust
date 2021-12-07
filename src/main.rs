#![allow(dead_code)]

mod helpers;

use crate::helpers::{PartialSum, StringUtils};

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "./inputs/day7/input.txt";

// There is surely a better way to do this
fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let squid_positions: Vec<u32> = line
        .split(",")
        .filter_map(|digit| digit.trim().safe_parse::<u32>())
        .collect();

    let min: u32 = *squid_positions.iter().min().unwrap();
    let max: u32 = *squid_positions.iter().max().unwrap() + 1;

    // memoize sums
    let mut sums = PartialSum::new();

    // track min total distance
    let mut min_distance = u32::MAX;

    for base_position in min..max {
        let mut current_distance: u32 = 0;
        for j in 0..squid_positions.len() {
            let distance: u32 = (base_position as i32 - squid_positions[j] as i32)
                .abs()
                .try_into()
                .unwrap();

            current_distance += sums.get(distance);
        }
        if current_distance < min_distance {
            min_distance = current_distance;
        }
    }
    println!("min distance -> {}", min_distance);
}
