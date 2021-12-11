#![allow(dead_code)]
#![allow(unused)]

mod helpers;

use std::cmp::Ordering::{self, Less};
use std::collections::{HashMap, HashSet};
use std::{fmt, fmt::Debug, fs::File, io::BufRead, io::BufReader};

use crate::helpers::print_2d;

const FILENAME: &str = "./inputs/day_11/input.txt";

// maybe point arrays where better? You can also destruture and patter match.

#[derive(Debug)]
struct Cave {
    positions: Vec<Vec<u32>>,
    total_flashes: u32,
    flashed: HashSet<Position>,
}

type Position = (usize, usize);

impl Cave {
    fn new(positions: Vec<Vec<u32>>) -> Self {
        Self {
            positions,
            total_flashes: 0,
            flashed: HashSet::new(),
        }
    }

    fn day(&mut self) {
        self.flashed.clear();

        for i in 0..self.positions.len() {
            for j in 0..self.positions[i].len() {
                self.increment((i, j));
            }
        }
    }

    fn increment(&mut self, position: Position) {
        let (i, j) = position;

        if self.flashed.contains(&position) {
            return;
        }

        if self.positions[i][j] == 9 {
            self.flashed.insert(position);
            self.total_flashes += 1;
            self.get_adjacent(position)
                .iter()
                .for_each(|adj| self.increment(*adj));
            self.positions[i][j] = 0;
        } else {
            self.positions[i][j] += 1;
        }
    }

    fn all_flashed(&self) -> bool {
        for row in &self.positions {
            for value in row {
                if value != &0 {
                    return false;
                }
            }
        }
        return true;
    }

    fn get_adjacent(&self, point: Position) -> Vec<Position> {
        let (x, y) = point;
        let length_x = *&self.positions.len() as i64;
        let length_y = *&self.positions[y].len() as i64;

        let mut adjacent: Vec<Position> = Vec::new();

        let test: [i64; 3] = [-1, 0, 1];
        for dx in test {
            for dy in test {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let new_x = (x as i64) + dx;
                let new_y = (y as i64) + dy;

                if new_x < 0 || new_y < 0 || new_x >= length_x || new_y >= length_y {
                    continue;
                }

                adjacent.push((new_x as usize, new_y as usize));
            }
        }

        adjacent
    }
}

fn main() {
    let file = File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);

    let positions: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|character| character.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut cave = Cave::new(positions);

    let mut all_flashed = cave.all_flashed();
    let mut day = 0;
    while !all_flashed {
        cave.day();
        day += 1;
        all_flashed = cave.all_flashed();
    }
    println!("{}", day);
}
