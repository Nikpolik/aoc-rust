#![allow(dead_code)]

mod helpers;

use itertools::{iproduct, Itertools};

use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

const FILENAME: &str = "./inputs/day8/test.txt";

fn parse_line(line: Result<String, Error>) -> String {
    let normal_string = line.unwrap();
    let output = normal_string.split(" | ").skip(1).next().unwrap();
    // Alocate string to return
    String::from(output)
}

fn decode(chars: &Vec<char>) -> u32 {
    if chars.len() == 7 {
        return 8;
    }
    if chars.len() == 4 {
        return 7;
    }
    if chars.len() == 3 {
        return 4;
    }
    if chars.len() == 2 {
        return 1;
    }

    // for permuation in ALLOWED_CHARS {
    //     let mapped_chars = chars
    //         .iter()
    //         .map(|c| {})
    //         .collect()
    //         .map(|mut chars: Vec<char>| {
    //             chars.sort();
    //             println!("{:?} mapped", chars);
    //             match chars[..] {
    //                 ['a', 'b', 'c', 'd', 'e', 'f', 'g'] => 8, // 8
    //                 ['b', 'c', 'd', 'e', 'f'] => 5,           // 5
    //                 ['a', 'c', 'd', 'f', 'g'] => 2,           // 2
    //                 ['a', 'b', 'c', 'd', 'f'] => 3,           // 3
    //                 ['a', 'b', 'd'] => 7,                     // 7
    //                 ['a', 'b', 'c', 'd', 'e', 'f'] => 9,      // 9
    //                 ['b', 'c', 'd', 'e', 'f', 'g'] => 6,      // 6
    //                 ['a', 'b', 'c', 'd', 'e', 'g'] => 4,      // 4
    //                 ['a', 'b'] => 0,                          // 1
    //                 _ => panic!("Could not decode value"),
    //             }
    //         });
    // }
    panic!("");
}

fn solve(output: &String) -> u32 {
    let chars: Vec<Vec<char>> = output
        .split(" ")
        .map(|digit| (*digit).chars().collect::<Vec<char>>())
        .collect();

    let total: u32 = chars
        .iter()
        .map(decode)
        .collect::<Vec<u32>>()
        .iter()
        .rev()
        .enumerate()
        .map(|(power, digit)| 10_u32.pow(power as u32) * *digit)
        .sum();

    println!("{:?}", total);
    total as u32
}

fn build_permutations() {
    for (a, b) in iproduct!(
        ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        ['a', 'b', 'c', 'd', 'e', 'f', 'g']
    ) {
        println!("{} {}", a, b);
    }
}

fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);

    let outputs: Vec<String> = reader.lines().map(parse_line).collect();
    let total: u32 = outputs.iter().map(solve).sum();

    println!("-------");
    println!("{}", total);
}
