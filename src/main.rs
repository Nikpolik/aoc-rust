use std::{
    fs,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "./inputs/day2/part_2.txt";

fn parse_line(line: String) -> (String, i32) {
    let mut split = line.split(' ');
    let command = split.next().unwrap();
    let amount = split.next().unwrap().parse::<i32>().unwrap();
    (command.to_string(), amount)
}

fn main() {
    let file = fs::File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;

    reader
        .lines()
        .filter_map(|f| match f {
            Ok(v) => Some(parse_line(v)),
            Err(_) => None,
        })
        .for_each(|(direction, amount)| match direction.as_str() {
            "forward" => {
                x += amount;
                y += aim * amount
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("unknow move"),
        });

    println!("{}", x * y);
}
