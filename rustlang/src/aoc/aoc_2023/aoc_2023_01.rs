use crate::aoclib;

#[derive(Default)]
pub struct Aoc2023_01 {
    calibration_values: Vec<i32>,
}

impl Aoc2023_01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl aoclib::Solution for Aoc2023_01 {
    fn id(&self) -> (i32, i32) {
        (2023, 1)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-01.txt");
        let nums = vec![
            "one", "1",
            "two", "2",
            "three", "3",
            "four", "4",
            "five", "5",
            "six", "6",
            "seven", "7",
            "eight", "8",
            "nine", "9",
        ];

        self.calibration_values = lines.iter().map(|line| {
            let mut calibration_value: i32 = 0;

            let first = line.chars().into_iter().enumerate().find_map(|(i, _)| {
                for (j, num) in nums.iter().enumerate() {
                    if i + num.len() > line.len() {
                        continue;
                    }

                    let found = &line[i..i + num.len()];

                    if *found == **num {
                        return Some(j as i32 / 2 + 1);
                    }
                }

                None
            });

            match first {
                None => panic!("invalid input"),
                Some(i) => calibration_value = calibration_value * 10 + i
            }

            let second = line.chars().rev().into_iter().enumerate().find_map(|(i, _)| {
                for (j, num) in nums.iter().enumerate() {
                    if (line.len() as i32 - i as i32 - num.len() as i32) < 0 {
                        continue;
                    }

                    let start = line.len() - i - num.len();
                    let end = line.len() - i;
                    let found = &line[start..end];

                    if found == *num {
                        return Some(j as i32 / 2 + 1);
                    }
                }

                None
            });

            match second {
                None => panic!("invalid input"),
                Some(i) => calibration_value = calibration_value * 10 + i
            }

            calibration_value
        }).collect();
    }

    fn part_one(&mut self) -> Vec<String> {
        let total: i32 = self.calibration_values.iter().sum();
        aoclib::output(total)
    }

    fn part_two(&mut self) -> Vec<String> {
        let total: i32 = self.calibration_values.iter().sum();
        aoclib::output(total)
    }
}