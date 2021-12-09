#![allow(dead_code)]

mod helpers;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

const FILENAME: &str = "./inputs/day8/input.txt";

fn parse_line(line: Result<String, Error>) -> String {
    let normal_string = line.unwrap();
    let output = normal_string.split(" | ").skip(1).next().unwrap();
    // Alocate string to return
    String::from(output)
}

// a  b  c    d    e    f    g
//[a, b, nil, nil, nil, nil, nil]
//['f', 'd', 'g', 'a', 'c', 'b', 'e'] => true,
//['a', 'b', 'c', 'd', 'e', 'f', 'g']
// gcbe

fn solve(output: &String) -> u32 {
    let total = output
        .split(" ")
        .map(|digit| digit.len())
        .filter(|count| match count {
            7 => true,
            4 => true,
            3 => true,
            2 => true,
            _ => false,
        })
        .count();
    total as u32
}

fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);

    let outputs: Vec<String> = reader.lines().map(parse_line).collect();
    let total: u32 = outputs.iter().map(solve).sum();

    println!("-------");
    println!("{}", total);
}
