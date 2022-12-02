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

const FILENAME: &str = "./inputs/2022/day_2/input.txt";

// ROCK A
// PAPER B
// SCIZORS C
// X LOSE
// Y DRAW
// Z WIN

fn get_move_score(current_move: &str) -> u64 {
    match current_move {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => {
            panic!("Move was not one of A, B, C")
        }
    }
}

fn get_move<'a>(opponent_move: &'a str, outcome: &'a str) -> &'a str {
    match (opponent_move, outcome) {
        ("B", "X") => "A",
        ("C", "Z") => "A",
        ("A", "Z") => "B",
        ("C", "X") => "B",
        ("A", "X") => "C",
        ("B", "Z") => "C",
        (any, "Y") => any,
        _ => panic!("Move not valid"),
    }
}

fn get_game_score(outcome: &str) -> u64 {
    match outcome {
        "Y" => 3,
        "Z" => 6,
        _ => 0,
    }
}

fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);
    let mut score: u64 = 0;
    reader.lines().for_each(|line| {
        let moves: Vec<String> = line
            .unwrap()
            .split(" ")
            .take(2)
            .map(|letter| letter.to_string())
            .collect();

        let first = moves.get(0).unwrap();
        let second = moves.get(1).unwrap();
        let my_move = get_move(first, second);
        let game_score = get_game_score(second);
        let move_score = get_move_score(my_move);

        println!(
            "{} {} | Score move : {} game: {} total: {}",
            first,
            second,
            move_score,
            game_score,
            game_score + move_score
        );
        score += game_score + move_score;
    });
    println!("{}", score);
}
