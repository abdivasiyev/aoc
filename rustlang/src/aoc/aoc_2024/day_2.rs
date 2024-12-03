use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_02 {
    nums: Vec<Vec<i64>>,
}

impl Aoc2024_02 {
    pub fn new() -> Self {
        Default::default()
    }
}

impl aoclib::Solution for Aoc2024_02 {
    fn id(&self) -> (i32, i32) {
        (2024, 2)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024_2.txt");
        self.nums = aoclib::numbers(lines);
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut count = 0;

        for report in &self.nums {
            if is_safe(report) {
                count += 1;
            }
        }
        aoclib::output(count)
    }

    fn part_two(&mut self) -> Vec<String> {
        let mut count = 0;

        for report in &self.nums {
            if is_safe(report) {
                count += 1;
                continue;
            }

            for i in 0..report.len() {
                let mut report = report.clone();
                report.remove(i);

                if is_safe(&report) {
                    count += 1;
                    break;
                }
            }
        }

        aoclib::output(count)
    }
}

fn is_safe(nums: &[i64]) -> bool {
    nums.is_sorted_by(|a, b| a < b && (1..=3).contains(&(b - a)))
        || nums.is_sorted_by(|a, b| a > b && (1..=3).contains(&(a - b)))
}
