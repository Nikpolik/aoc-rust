#![allow(dead_code)]

use std::{collections::HashMap, str::FromStr};

pub type Point = (usize, usize);

pub fn get_adjacent(point: Point, length_x: usize, length_y: usize) -> Vec<Point> {
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

pub fn print_2d<T: std::fmt::Display>(items: &Vec<Vec<T>>) {
    for _ in items[0].iter() {
        print!("----");
    }
    println!("");
    for row in items.iter() {
        for col in row.iter() {
            print!(" {} |", col);
        }
        println!("");
        for _ in row.iter() {
            print!("----");
        }
        println!("");
    }
}

// memoize
pub struct PartialSum {
    partial_sums: HashMap<u32, u32>,
}

impl PartialSum {
    pub fn new() -> Self {
        Self {
            partial_sums: HashMap::new(),
        }
    }

    pub fn get(&mut self, finish: u32) -> u32 {
        if finish == 0 {
            return 0;
        }; // base case

        if self.partial_sums.contains_key(&finish) {
            return *self.partial_sums.get(&finish).unwrap();
        };

        let result = finish + self.get(finish - 1);
        self.partial_sums.insert(finish, result);
        result
    }
}

pub trait StringUtils {
    fn safe_parse<T: FromStr>(&self) -> Option<T>;
}

impl StringUtils for String {
    fn safe_parse<T: FromStr>(&self) -> Option<T> {
        match self.parse::<T>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}

impl StringUtils for str {
    fn safe_parse<T: FromStr>(&self) -> Option<T> {
        match self.parse::<T>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}
