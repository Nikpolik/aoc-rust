#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    collections::{hash_map, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

use crate::helpers::StringUtils;

type BingoBoard = HashMap<u32, Vec<(usize, usize)>>;

#[derive(Debug)]
pub struct BingoGame {
    board: BingoBoard,
    marked: Vec<Vec<bool>>,
}

impl BingoGame {
    fn new(board: BingoBoard, marked: Vec<Vec<bool>>) -> Self {
        Self { board, marked }
    }

    pub fn from_reader(buffer: &mut BufReader<File>) -> Option<Self> {
        let mut board: BingoBoard = BingoBoard::new();
        let mut marked: Vec<Vec<bool>> = Vec::new();
        let mut line = String::new();
        let mut row: usize = 0;

        loop {
            let bytes_read = buffer.read_line(&mut line).expect("Could not read line");
            if bytes_read == 0 {
                break;
            }

            let mut count: usize = 0;
            line.split_whitespace()
                .filter_map(|digit| digit.safe_parse::<u32>())
                .enumerate()
                .for_each(|(index, digit)| {
                    if !board.contains_key(&digit) {
                        board.insert(digit, Vec::new());
                    }
                    board.get_mut(&digit).unwrap().push((row, index));
                    count += 1;
                });

            line.clear();
            if count == 0 {
                break;
            }

            marked.push(vec![false; count + 1]);
            row += 1;
        }

        if board.len() == 0 {
            return None;
        };

        Some(BingoGame::new(board, marked))
    }

    // return -1 if we dont win
    pub fn won(&self) -> bool {
        for i in 0..self.marked.len() - 1 {
            let mut won_x = true;
            let mut won_y = true;
            for j in 0..self.marked[0].len() - 1 {
                if !self.marked[i][j] {
                    won_x = false;
                }
                if !self.marked[j][i] {
                    won_y = false;
                }
            }
            if won_x || won_y {
                return true;
            }
        }
        false
    }

    pub fn calculate_total(&self) -> u32 {
        let total: u32 = self
            .board
            .iter()
            .flat_map(|(digit, coordinates)| {
                coordinates.iter().filter_map(|(x, y)| {
                    if !self.marked[*x][*y] {
                        // this is so true we must clone it to return multiple times
                        // rust is smart and dumb at the same time god
                        Some(digit.clone())
                    } else {
                        None
                    }
                })
            })
            .sum();
        return total;
    }

    // return if marked
    pub fn mark(&mut self, number: u32) -> bool {
        let coordinates = match self.board.get(&number) {
            Some(coordinates) => coordinates,
            None => return false,
        };
        coordinates.iter().for_each(|(x, y)| {
            self.marked[*x][*y] = true;
        });
        true
    }

    pub fn print_board(&self) {
        for i in 0..self.marked.len() - 1 {
            for j in 0..self.marked[0].len() - 1 {
                let digit =
                    match self.board.iter().find(|(.., coordinates)| {
                        coordinates.iter().any(|(x, y)| *x == i && *y == j)
                    }) {
                        Some((digit, ..)) => digit,
                        None => panic!("i: {}, j: {}, With Board -> \n{:?}", i, j, self.board),
                    };
                if self.marked[i][j] {
                    print!(" {}* ", digit);
                } else {
                    print!(" {}. ", digit)
                }
            }
            print!("\n");
        }
    }
}
