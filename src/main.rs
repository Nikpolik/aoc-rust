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

fn sort(mut numbers: [usize; 2]) -> [usize; 2] {
    numbers.sort();
    numbers
}

fn parse_line(line: &String) -> Option<Vec<(usize, usize)>> {
    let mut split = line.split(" -> ");
    let (start_x, start_y) = parse_tuple(split.next().unwrap());
    let (end_x, end_y) = parse_tuple(split.next().unwrap());

    // non horizontal or vertical line return
    if start_x != end_x && start_y != end_y {
        return None;
    }

    if start_x == end_x {
        let [start_y, end_y] = sort([start_y, end_y]);
        return Some((start_y..(end_y + 1)).map(|y| (start_x, y)).collect());
    }

    let [start_x, end_x] = sort([start_x, end_x]);
    Some((start_x..(end_x + 1)).map(|x| (x, start_y)).collect())
}

fn parse_tuple(tuple_string: &str) -> (usize, usize) {
    let coordinates: Vec<usize> = tuple_string
        .split(",")
        .filter_map(|digit| digit.safe_parse::<usize>())
        .collect();
    (coordinates[0], coordinates[1])
}

const FILENAME: &str = "./inputs/day5/input.txt";
// There is surely a better way to do this
fn main() {
    let file = fs::File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);
    let mut scores: HashMap<(usize, usize), u32> = HashMap::new();
    reader
        .lines()
        .filter_map(|res| match res {
            Ok(v) => parse_line(&v),
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
