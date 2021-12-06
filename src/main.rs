#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;

mod bingo;
mod helpers;
use crate::bingo::BingoGame;
use crate::helpers::StringUtils;

use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    process::exit,
};

const FILENAME: &str = "./inputs/day4/input.txt";
// There is surely a better way to do this
fn main() {
    let file = fs::File::open(FILENAME).expect("Could not read file");
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    reader
        .read_line(&mut first_line)
        .expect("Could not read first line from input");

    let rng: Vec<u32> = first_line
        .split(",")
        .filter_map(|digit| digit.safe_parse::<u32>())
        .collect();

    // skip empty line
    reader.read_line(&mut first_line).unwrap();

    let mut games: Vec<BingoGame> = Vec::new();
    while let Some(game) = BingoGame::from_reader(&mut reader) {
        games.push(game);
    }
    rng.iter().for_each(|number| {
        games.iter_mut().for_each(|game| {
            game.mark(*number);
        });
        if games.len() == 1 {
            let last_game = games.first().unwrap();
            if last_game.won() {
                println!(
                    "{}, score: {}",
                    number,
                    last_game.calculate_total() * number
                );
            }
        }
        games.retain(|game| !game.won());
    });
}
