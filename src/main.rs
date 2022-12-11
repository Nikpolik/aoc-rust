mod helpers;
use crate::helpers::StringUtils;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Input {
    action: Move,
    times: i32,
}

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => unreachable!(),
        }
    }
}

impl FromStr for Input {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let direction = parts.next().unwrap().parse::<Move>().unwrap();
        let amount = parts.next().unwrap().parse::<i32>()?;

        Ok(Self {
            action: direction,
            times: amount,
        })
    }
}

struct Rope {
    head: Point,
    pub tails: [Point; 9],
    pub solution: SolutionType,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            head: Point::new(0, 0),
            tails: [Point::new(0, 0); 9],
            solution: HashSet::new(),
        }
    }

    pub fn print_solution(&self) {
        let max_x = self.solution.iter().map(|i| i.x).max().unwrap();
        let min_x = self.solution.iter().map(|i| i.x).min().unwrap();
        let max_y = self.solution.iter().map(|i| i.y).max().unwrap();
        let min_y = self.solution.iter().map(|i| i.y).min().unwrap();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if self.solution.contains(&Point::new(x, y)) {
                    print!(" * ");
                } else {
                    print!(" . ");
                }
            }
            println!("");
        }
    }

    pub fn process_input(&mut self, rope_move: Input) {
        for _ in 0..rope_move.times {
            self.move_head(&rope_move.action);
        }
    }

    pub fn move_head(&mut self, rope_move: &Move) {
        match rope_move {
            Move::Up => self.head.y += 1,
            Move::Down => self.head.y -= 1,
            Move::Right => self.head.x += 1,
            Move::Left => self.head.x -= 1,
        }

        for i in 0..9 {
            self.move_tail(i);
        }
        println!("{}", self);
    }

    fn move_tail(&mut self, index: usize) {
        let prev = match index {
            0 => self.head,
            _ => self.tails[index - 1],
        };
        let x_distance = prev.x - self.tails[index].x;
        let y_distance = prev.y - self.tails[index].y;
        if x_distance.abs() > 1 {
            let direction = if x_distance >= 0 { -1 } else { 1 };

            self.tails[index].x = prev.x + direction;
            self.tails[index].y = prev.y
        }

        if y_distance.abs() > 1 {
            let direction = if y_distance >= 0 { -1 } else { 1 };
            self.tails[index].y = prev.y + direction;
            self.tails[index].x = prev.x
        }

        if index == 8 {
            self.solution.insert(self.tails[index]);
        }
    }
}

impl std::fmt::Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let max_x = self.tails.iter().map(|i| i.x).max().unwrap();
        let min_x = self.tails.iter().map(|i| i.x).min().unwrap();
        let max_y = self.tails.iter().map(|i| i.y).max().unwrap();
        let min_y = self.tails.iter().map(|i| i.y).min().unwrap();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if self.tails.contains(&Point::new(x, y)) {
                    write!(f, " * ")?
                } else {
                    write!(f, " . ")?
                }
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

type InpuType = Input;
type SolutionType = HashSet<Point>;

fn main() {
    let input_str = include_str!("../inputs/2022/day9/input.txt");
    let inputs: Vec<InpuType> = input_str
        .lines()
        .into_iter()
        .filter_map(|s| s.safe_parse())
        .collect();

    let mut rope = Rope::new();
    for input in inputs {
        rope.process_input(input);
    }
    println!("{}", rope.solution.len());
}
