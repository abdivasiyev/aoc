use itertools::Itertools;
use std::fmt::Display;
use std::fs::read_to_string;

pub trait Solution {
    fn id(&self) -> (i32, i32);
    fn parse(&mut self);
    fn part_one(&mut self) -> Vec<String>;
    fn part_two(&mut self) -> Vec<String>;
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

        solution.parse();

        let output_one = solution.part_one();
        let output_two = solution.part_two();

        println!("\n{year} - Day {day}\n", year = year, day = day);
        println!("Part 1: {output}", output = output_one.join(" "));
        println!("Part 2: {output}\n", output = output_two.join(" "));
    }
}

pub fn read_lines(path: &str) -> Vec<String> {
    let mut lines = vec![];

    for line in read_to_string(path).unwrap().lines() {
        lines.push(line.to_string());
    }

    return lines;
}

pub fn numbers(lines: Vec<String>) -> Vec<Vec<i64>> {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn output<T: Display>(arg: T) -> Vec<String> {
    vec![format!("{}", arg)]
}

pub fn combinations(items: &Vec<String>, n: usize) -> Vec<String> {
    (2..n).fold(
        items
            .iter()
            .cartesian_product(items.iter())
            .map(|(a, b)| a.to_owned() + b)
            .collect(),
        |acc, _| {
            acc.into_iter()
                .cartesian_product(items.iter())
                .map(|(a, b)| a.to_owned() + b)
                .collect()
        },
    )
}
