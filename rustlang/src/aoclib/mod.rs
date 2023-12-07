use std::fmt::Display;
use std::fs::read_to_string;
use std::time::Instant;

pub trait Solution {
    fn id(&self) -> (i32, i32);
    fn parse(&mut self);
    fn part_one(&mut self) -> String;
    fn part_two(&mut self) -> String;
}

pub enum Mode {
    All,
    Single(i32, i32),
    Last,
}

pub struct Runner {
    mode: Mode,
    solutions: Vec<Box<dyn Solution>>,
}

impl Runner {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            solutions: vec![],
        }
    }

    pub fn add(&mut self, solution: Box<dyn Solution>) {
        self.solutions.push(solution);
    }

    pub fn run(&mut self) {
        match self.mode {
            Mode::All => {
                for solution in &mut self.solutions {
                    Self::print_solution(solution)
                }
            }
            Mode::Single(year, day) => {
                if let Some(solution) = self.solutions.iter_mut().find(|solution| {
                    let (y, d) = solution.id();
                    return year == y && day == d;
                }) {
                    Self::print_solution(solution);
                }
            }
            Mode::Last => {
                if let Some(solution) = self.solutions.last_mut() {
                    Self::print_solution(solution);
                }
            }
        }
    }

    fn print_solution(solution: &mut Box<dyn Solution>) {
        let (year, day) = solution.id();

        let parsing_started = Instant::now();
        solution.parse();
        let parsing_elapsed = parsing_started.elapsed().as_millis();

        let part_one_started = Instant::now();
        let output_one = solution.part_one();
        let part_one_elapsed = part_one_started.elapsed().as_millis();

        let part_two_started = Instant::now();
        let output_two = solution.part_two();
        let part_two_elapsed = part_two_started.elapsed().as_millis();

        println!("**Year {year:04} - Day {day:02}**\n\n");
        println!("Parsing [{parsing_elapsed} ms]\n");
        println!("Part 1  [{part_one_elapsed} ms]:{:14}{output_one}", "");
        println!("Part 2  [{part_two_elapsed} ms]:{:14}{output_two}", "");
        println!("\n\n");
    }
}

pub fn hello() {
    println!("Hello from library!");
}

pub fn read_lines(path: &str) -> Vec<String> {
    let mut lines = vec![];

    for line in read_to_string(path).unwrap().lines() {
        lines.push(line.to_string());
    }

    return lines;
}

pub fn output<T: Display>(arg: T) -> String {
    format!("{}", arg)
}