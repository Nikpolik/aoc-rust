#![allow(dead_code)]

mod helpers;

use std::cmp::Ordering::{self, Less};
use std::collections::{HashMap, HashSet};
use std::{fmt, fmt::Debug, fs::File, io::BufRead, io::BufReader};

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
        print!("---------");
    }
    println!("");
    for row in items.iter() {
        for col in row.iter() {
            print!(" {} |", col);
        }
        println!("");
        for _ in row.iter() {
            print!("---------");
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

    let mut low_points: Vec<Point> = Vec::new();

    for (x, cols) in rows.iter().enumerate() {
        for (y, value) in cols.iter().enumerate() {
            if *value == 9 {
                continue;
            };

            let adjacent = get_adjacent((x, y), rows.len(), cols.len());
            let low_point = adjacent.iter().all(|(x, y)| {
                let other_value = rows[*x][*y];
                other_value.cmp(value) == Ordering::Greater
            });

            if low_point {
                low_points.push((x, y));
            }
        }
    }

    let mut results: Vec<usize> = Vec::new();

    for start in low_points.iter() {
        let (x, _y) = *start;
        let mut to_check: Vec<Point> = get_adjacent(*start, rows.len(), rows[x].len())
            .into_iter()
            .filter(|(x, y)| rows[*x][*y] != 9)
            .collect();

        let mut visited: HashSet<Point> = HashSet::from_iter(to_check.clone().into_iter());
        visited.insert(*start);

        while to_check.len() > 0 {
            let next = to_check.pop().unwrap();
            let (other_x, _) = next;

            for adjacent in get_adjacent(next, rows.len(), rows[other_x].len()) {
                let (z_x, z_y) = adjacent;
                let value = rows[z_x][z_y];
                if value == 9 {
                    continue;
                }

                if visited.contains(&adjacent) {
                    continue;
                }

                visited.insert(adjacent);
                to_check.push(adjacent);
            }
        }

        results.push(visited.len());
    }

    results.sort();
    let total = results
        .into_iter()
        .rev()
        .take(3)
        .reduce(|value, acc| value * acc)
        .unwrap();

    println!("Total {}", total);
}
