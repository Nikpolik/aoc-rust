use itertools::Itertools;
use std::ops::BitXor;
use std::{
    fs,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "./inputs/day3/input.txt";

fn find_common_at_position(data: &Vec<Vec<u32>>, position: usize, search_type: bool) -> u32 {
    let digit_sum: u32 = data.iter().map(|data| data[position]).sum();
    let one_common = digit_sum / (data.len() as u32 - digit_sum) > 0;
    !BitXor::bitxor(one_common, search_type) as u32
}

fn solve(data: &mut Vec<Vec<u32>>, search_type: bool) -> String {
    let mut position: usize = 0;
    while data.len() > 1 {
        let common = find_common_at_position(&data, position, search_type);
        data.retain(|bits| bits[position] == common);
        position += 1;
    }
    data.pop().unwrap().iter().join("")
}

// There is surely a better way to do this
fn main() {
    let file = fs::File::open(FILENAME).expect("Could not read file");
    let reader = BufReader::new(file);
    let mut total = 0;

    let data: Vec<Vec<u32>> = reader
        .lines()
        .filter_map(|f| match f {
            Ok(v) => {
                total += 1;
                let chars: Vec<u32> = v.chars().filter_map(|c| c.to_digit(10)).collect();
                Some(chars)
            }
            Err(_) => None,
        })
        .collect();

    let oxygen = solve(&mut data.clone(), true);
    let scrubber = solve(&mut data.clone(), false);
    println!("Oxygen : {} CO2, Scrubber: {}", oxygen, scrubber);
}
