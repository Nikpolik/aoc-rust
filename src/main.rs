#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "./inputs/day6/input.txt";
// There is surely a better way to do this
fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);

    // hold num of fishes in each state of their life
    let mut fishes: [u64; 9] = [0; 9];

    reader.lines().for_each(|result| {
        if let Ok(line) = result {
            line.split(",")
                .into_iter()
                .map(|digit| digit.parse::<usize>().unwrap())
                .for_each(|digit| fishes[digit] += 1)
        }
    });

    for _ in 0..256 {
        let total_done = fishes[0];

        for digit in 1..9 {
            fishes[digit - 1] = fishes[digit];
        }

        fishes[6] += total_done;
        fishes[8] = total_done;
    }
    print!("{}", fishes.iter().sum::<u64>());
}
