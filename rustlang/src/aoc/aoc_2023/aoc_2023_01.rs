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

        self.calibration_values = lines.iter().map(|line| {
            let mut calibration_value: i32 = 0;

            let first = line.chars().into_iter().find(|c| c.is_ascii_digit()).map(|c| (c as i32) - ('0' as i32));

            match first {
                None => {}
                Some(i) => calibration_value = calibration_value * 10 + i
            }

            let second = line.chars().rev().into_iter().find(|c| c.is_ascii_digit()).map(|c| (c as i32) - ('0' as i32));

            match second {
                None => {}
                Some(i) => calibration_value = calibration_value * 10 + i
            }

            calibration_value
        }).collect();
    }

    fn part_one(&mut self) -> String {
        let total: i32 = self.calibration_values.iter().sum();
        aoclib::output(total)
    }

    fn part_two(&mut self) -> String {
        aoclib::output("unsolved")
    }
}