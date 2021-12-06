#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::{zip, Itertools};

mod bingo;
mod helpers;
use crate::bingo::BingoGame;
use crate::helpers::StringUtils;

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
    ops::Range,
    process::exit,
};

fn sort(mut numbers: [u32; 2]) -> [u32; 2] {
    numbers.sort();
    numbers
}

fn parse_line(line: &String) -> Vec<(u32, u32)> {
    let mut split = line.split(" -> ");
    let (mut start_x, mut start_y) = parse_tuple(split.next().unwrap());
    let (end_x, end_y) = parse_tuple(split.next().unwrap());

    if start_x == end_x {
        let [start_y, end_y] = sort([start_y, end_y]);
        return (start_y..(end_y + 1)).map(|y| (start_x, y)).collect();
    }

    if start_y == end_y {
        let [start_x, end_x] = sort([start_x, end_x]);
        return (start_x..(end_x + 1)).map(|x| (x, start_y)).collect();
    }

    let mut slots = Vec::new();
    slots.push((start_x.clone(), start_y));
    loop {
        start_x = get_next(start_x, end_x);
        start_y = get_next(start_y, end_y);
        slots.push((start_x.clone(), start_y));
        if start_x == end_x || start_y == end_y {
            break;
        }
    }
    slots
}

fn get_next(start: u32, end: u32) -> u32 {
    if start > end {
        start - 1
    } else {
        start + 1
    }
}

fn parse_tuple(tuple_string: &str) -> (u32, u32) {
    let coordinates: Vec<u32> = tuple_string
        .split(",")
        .filter_map(|digit| digit.safe_parse::<u32>())
        .collect();
    (coordinates[0], coordinates[1])
}

const FILENAME: &str = "./inputs/day5/input";
// There is surely a better way to do this
fn main() {
    let file = fs::File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);
    let mut scores: HashMap<(u32, u32), u32> = HashMap::new();
    reader
        .lines()
        .filter_map(|res| match res {
            Ok(v) => {
                let result = parse_line(&v);
                println!("{}", v);
                println!("{:?}", result);
                Some(result)
            }
            Err(e) => panic!("{}", e),
        })
        .flatten()
        .for_each(|coordinates| {
            let coordinates_score = match scores.get(&coordinates) {
                Some(v) => v + 1,
                None => 1,
            };
            scores.insert(coordinates, coordinates_score);
        });
    let total = scores.values().filter(|v| **v > 1).count();
    println!("{}", total);
}
