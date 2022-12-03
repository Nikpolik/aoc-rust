#![allow(dead_code)]
#![allow(unused)]

mod helpers;

use std::cmp::Ordering::{self, Less};
use std::collections::HashMap;
use std::slice::SliceIndex;
use std::{fmt, fmt::Debug, fs::File, io::BufRead, io::BufReader};

use itertools::Itertools;

use crate::helpers::print_2d;
use crate::helpers::StringUtils;

const FILENAME: &str = "./inputs/2022/day3/test.txt";

fn get_letter_priorit(letter: char) -> usize {
    let a_capital = 65;
    let a = 97;
    let letter_code = letter as usize;

    if letter_code >= a {
        return 1 + letter_code - a;
    } else {
        return 27 + letter_code - a_capital;
    }
}

fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);
    let mut total_priority: u64 = 0;
    for group in reader.lines().chunks(3).into_iter() {
        let mut all_letters: [u8; 53] = [0; 53];
        for line in group {
            let mut current_letters: [bool; 53] = [false; 53];
            line.unwrap().chars().for_each(|letter| {
                let priority = get_letter_priorit(letter);
                current_letters[priority] = true;
            });
            for i in 0..current_letters.len() {
                if current_letters[i] {
                    all_letters[i] += 1;
                }
            }
        }

        for i in 0..all_letters.len() {
            if all_letters[i] == 3 {
                total_priority += i as u64;
            }
        }
    }
    println!("{}", total_priority);
}
