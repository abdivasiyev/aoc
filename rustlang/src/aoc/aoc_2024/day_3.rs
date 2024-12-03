use crate::aoclib;
extern crate regex;
use regex::Regex;

#[derive(Default)]
pub struct Aoc2024_03 {
    sequences: Vec<String>,
}

impl Aoc2024_03 {
    pub fn new() -> Self {
        Default::default()
    }
}

impl aoclib::Solution for Aoc2024_03 {
    fn id(&self) -> (i32, i32) {
        (2024, 3)
    }

    fn parse(&mut self) {
        self.sequences = aoclib::read_lines("input/2024_3.txt");
    }

    fn part_one(&mut self) -> Vec<String> {
        let regex = Regex::new(r"(?m)mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
        let mut sum = 0;

        for sequence in self.sequences.clone() {
            for (instruction, []) in regex.captures_iter(&sequence).map(|cap| cap.extract()) {
                let parts = instruction
                    .strip_prefix("mul(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split_once(",")
                    .unwrap();

                sum += parts.0.parse::<i64>().unwrap() * parts.1.parse::<i64>().unwrap()
            }
        }

        aoclib::output(sum)
    }

    fn part_two(&mut self) -> Vec<String> {
        let regex = Regex::new(r"(?m)mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
        let mut sum = 0;
        let mut enabled = true;

        for sequence in self.sequences.clone() {
            for (instruction, []) in regex.captures_iter(&sequence).map(|cap| cap.extract()) {
                match instruction {
                    "do()" => enabled = true,
                    "don't()" => enabled = false,
                    _ => {
                        if !enabled {
                            continue;
                        }
                        let parts = instruction
                            .strip_prefix("mul(")
                            .unwrap()
                            .strip_suffix(")")
                            .unwrap()
                            .split_once(",")
                            .unwrap();

                        sum += parts.0.parse::<i64>().unwrap() * parts.1.parse::<i64>().unwrap()
                    }
                }
            }
        }

        aoclib::output(sum)
    }
}
