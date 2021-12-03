use std::{
    fs,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

const FILENAME: &str = "./inputs/day3/input.txt";

fn sum_arrays(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    a.iter().zip(b).map(|(a, b)| a + b).collect()
}

// There is surely a better way to do this
fn main() {
    let file = fs::File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);
    let mut total = 0;

    let result = reader
        .lines()
        .filter_map(|f| match f {
            Ok(v) => {
                total += 1;
                let chars: Vec<u32> = v.chars().filter_map(|c| c.to_digit(10)).collect();
                Some(chars)
            }
            Err(_) => None,
        })
        .reduce(sum_arrays)
        .unwrap();

    let common: String = result
        .clone()
        .iter_mut()
        .map(|digit| if *digit / (total - *digit) > 0 { 1 } else { 0 })
        .join("");

    let uncommon: String = result
        .clone()
        .iter_mut()
        .map(|digit| if *digit / (total - *digit) > 0 { 0 } else { 1 })
        .join("");

    println!("{}", common);
    println!("{}", uncommon);
}
