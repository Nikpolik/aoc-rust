#![allow(dead_code)]
#![allow(unused)]

mod helpers;

use anyhow::{Error, Result};
use std::cmp::Ordering::{self, Less};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::slice::SliceIndex;
use std::str::FromStr;
use std::{fmt, fmt::Debug, fs::File, io::BufRead, io::BufReader};

use itertools::Itertools;

use crate::helpers::print_2d;
use crate::helpers::StringUtils;

const FILENAME: &str = "./inputs/2022/day4/input.txt";

#[derive(Debug)]
struct Pair {
    start: u8,
    end: u8,
}

impl Pair {
    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    pub fn overlapp(&self, other: &Self) -> bool {
        other.end <= self.end && other.end >= self.start
            || other.start <= self.end && other.start >= self.start
    }
}

impl FromStr for Pair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();

        let start_fromstr = start.parse::<u8>()?;
        let end_fromstr = end.parse::<u8>()?;

        Ok(Self {
            start: start_fromstr,
            end: end_fromstr,
        })
    }
}

fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);
    let groups = reader
        .lines()
        .map(|line| {
            line.unwrap().split_once(",").and_then(|(first, second)| {
                Some((
                    first.parse::<Pair>().unwrap(),
                    second.parse::<Pair>().unwrap(),
                ))
            })
        })
        .filter_map(|v| v)
        .filter(|(first, second)| first.overlapp(&second) || second.overlapp(&first))
        .count();
    println!("{:?}", groups);
}
