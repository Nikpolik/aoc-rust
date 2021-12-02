use itertools::{Itertools, TupleWindows};
use std::{
    fs,
    io::{BufRead, BufReader},
    slice::Iter,
};

const FILENAME: &str = "./inputs/day2/part_1.txt";

fn parse_line(line: String) -> (String, i32) {
    let mut split = line.split(' ');
    let command = split.next().unwrap();
    let amount = split.next().unwrap().parse::<i32>().unwrap();
    (command.to_string(), amount)
}

fn main() {
    let file = fs::File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);
    let mut x = 0;
    let mut y = 0;

    reader
        .lines()
        .filter_map(|f| match f {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .map(parse_line)
        .for_each(|(a, b)| match a.as_str() {
            "forward" => x += b,
            "down" => y += b,
            "up" => y -= b,
            _ => panic!("unknow move"),
        });

    println!("{}", x * y);
}
