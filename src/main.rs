#![allow(dead_code)]
#![allow(unused)]

mod helpers;

use std::cmp::Ordering::{self, Less};
use std::collections::{HashMap, HashSet};
use std::slice::SliceIndex;
use std::{fmt, fmt::Debug, fs::File, io::BufRead, io::BufReader};

use itertools::Itertools;

use crate::helpers::print_2d;

const FILENAME: &str = "./inputs/2022/day_1/input.txt";

fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);
}
