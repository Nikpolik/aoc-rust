mod helpers;
use crate::helpers::StringUtils;
use std::collections::VecDeque;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Noop,
    Add(i32),
}

#[derive(Debug)]
struct Job {
    command: Command,
    time: i32,
}

const METRIC_POINTS: [i32; 6] = [20, 60, 100, 140, 180, 220];

impl FromStr for Job {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let command = split.next().unwrap();
        match command {
            "noop" => Ok(Self {
                command: Command::Noop,
                time: 1,
            }),
            "addx" => {
                let value = split.next().unwrap().parse::<i32>().unwrap();
                Ok(Self {
                    command: Command::Add(value),
                    time: 2,
                })
            }
            _ => {
                panic!("Invalid command");
            }
        }
    }
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Task {{ command: {:?}, time: {} }}",
            self.command, self.time
        )
    }
}

struct CPU {
    tasks: VecDeque<Job>,
    addr: i32,
    pub clock: i32,
    metrics: Vec<i32>,
    screen_width: u32,
    screen_height: u32,
    pos_x: i32,
    pos_y: u32,
}

impl CPU {
    fn new(width: u32, height: u32, program: VecDeque<Job>) -> CPU {
        CPU {
            tasks: program,
            addr: 1,
            clock: 0,
            metrics: Vec::new(),
            screen_width: width,
            screen_height: height,
            pos_x: 0,
            pos_y: 0,
        }
    }

    fn run(&mut self) {
        while let Some(job) = &mut self.tasks.pop_front() {
            // println!("{}", job);
            // tick clock until job is finished
            for _ in 0..job.time {
                self.clock += 1;
                // capture metrics
                if METRIC_POINTS.contains(&self.clock) {
                    // println!("{} {}", self.clock, self.addr);
                    self.metrics.push(self.clock * self.addr);
                }

                self.draw();
            }
            match job.command {
                Command::Add(v) => {
                    self.addr += v;
                    // println!("Executing Add {} -> {}", v, self.addr);
                }
                Command::Noop => {}
            }
        }
    }

    fn get_strength(&mut self) -> i32 {
        println!("{:?} {}", self.metrics, self.clock);
        self.metrics.iter().sum()
    }

    fn draw(&mut self) {
        let current_sprite: [i32; 3] = [self.addr - 1, self.addr, self.addr + 1];
        if current_sprite.contains(&self.pos_x) {
            print!("#");
        } else {
            print!(".");
        }
        self.pos_x += 1;
        if self.pos_x as u32 == self.screen_width {
            println!("");
            self.pos_x = 0;
            self.pos_y = (self.pos_y + 1) % self.screen_height;
        }
    }
}

pub fn main() {
    let input_str = include_str!("../inputs/2022/day10/input.txt");
    let program: VecDeque<Job> = input_str.lines().flat_map(|l| l.safe_parse()).collect();
    let mut cpu = CPU::new(40, 6, program);
    cpu.run();
    println!("{}", cpu.get_strength())
}
