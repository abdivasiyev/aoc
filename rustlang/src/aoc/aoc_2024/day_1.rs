use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_01 {
    nums: Vec<[i64; 2]>,
}

impl Aoc2024_01 {
    pub fn new() -> Self {
        Default::default()
    }
}

impl aoclib::Solution for Aoc2024_01 {
    fn id(&self) -> (i32, i32) {
        (2024, 1)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024_1.txt");
        let mut left_nums: Vec<i64> = Vec::new();
        let mut right_nums: Vec<i64> = Vec::new();

        for line in lines {
            let (left, right) = line.split_once("   ").unwrap();
            let left = left.parse::<i64>().unwrap();
            let right = right.parse::<i64>().unwrap();

            left_nums.push(left);
            right_nums.push(right);
        }

        left_nums.sort();
        right_nums.sort();

        self.nums.extend(left_nums.iter().zip(right_nums.iter()).map(|(l, r)| [*l, *r]));
    }

    fn part_one(&mut self) -> Vec<String> {
        aoclib::output(
            self.nums.iter().map(|[l, r]| (*l).abs_diff(*r) as i64).sum::<i64>(),
        )
    }

    fn part_two(&mut self) -> Vec<String> {
        let mut score = 0;

        for [l, _] in self.nums.iter() {
            let count = self.nums.iter().filter(|[_, r]| l == r).count() as i64;

            score += count * l;
        }

        aoclib::output(score)
    }
}
