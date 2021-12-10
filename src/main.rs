#![allow(dead_code)]

mod helpers;

use std::cmp::Ordering::{self, Less};
use std::{fmt::Debug, fs::File, io::BufRead, io::BufReader};

const FILENAME: &str = "./inputs/day9/input.txt";

type Point = (usize, usize);

// maybe point arrays where better? You can also destruture and patter match.
fn get_adjacent(point: Point, length_x: usize, length_y: usize) -> Vec<Point> {
    let (x, y) = point;

    let mut adjacent: Vec<Point> = Vec::new();

    if x > 0 {
        adjacent.push((x - 1, y));
    }

    if y > 0 {
        adjacent.push((x, y - 1));
    }

    if x < length_x - 1 {
        adjacent.push((x + 1, y));
    }

    if y < length_y - 1 {
        adjacent.push((x, y + 1));
    }

    adjacent
}

fn print_2d<T: std::fmt::Display>(items: &Vec<Vec<T>>) {
    for _ in items[0].iter() {
        print!("--");
    }
    println!("");
    for row in items.iter() {
        for col in row.iter() {
            print!("{}|", col);
        }
        println!("");
        for _ in row.iter() {
            print!("--");
        }
        println!("");
    }
}

fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);

    let mut rows: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let numbers: Vec<u32> = line
            .unwrap()
            .chars()
            .filter_map(|digit| digit.to_digit(10))
            .collect();

        rows.push(numbers);
    }

    print_2d(&rows);
    let mut score = 0;

    for (x, cols) in rows.iter().enumerate() {
        println!("{}", x);
        for (y, value) in cols.iter().enumerate() {
            println!("{} {}", x, y);
            let low_point = get_adjacent((x, y), rows.len(), cols.len())
                .iter()
                .all(|(x, y)| {
                    let other_value = rows[*x][*y];
                    other_value.cmp(value) == Ordering::Greater
                });
            if low_point {
                score += value + 1;
            }
        }
    }

    println!("{}", score);
}
