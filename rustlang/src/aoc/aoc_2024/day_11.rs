use crate::aoclib;
use std::collections::HashMap;

#[derive(Default)]
pub struct Aoc2024_11 {
    stones: Vec<i64>,
}

impl Aoc2024_11 {
    pub fn new() -> Self {
        Default::default()
    }

    fn split_even_len_num(&self, n: i64) -> (bool, i64, i64) {
        let mut num = n;
        let mut digits = Vec::new();

        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }

        if digits.len() % 2 != 0 {
            return (false, 0, 0);
        }

        let mut first = 0;
        let mut second = 0;
        let mut i = 0;

        digits.reverse();

        while i < digits.len() / 2 {
            first = digits[i] + first * 10;
            i += 1;
        }

        while i < digits.len() {
            second = digits[i] + second * 10;
            i += 1;
        }

        (true, first, second)
    }

    fn blink(&self, stones: &Vec<i64>, n: i64) -> Vec<i64> {
        if n == 0 {
            return stones.to_vec();
        }

        let mut new_stones: Vec<i64> = Vec::new();

        for stone in stones {
            if *stone == 0 {
                new_stones.push(1);
                continue;
            }

            let (ok, first, second) = self.split_even_len_num(*stone);

            if !ok {
                new_stones.push(stone * 2024);
                continue;
            }

            new_stones.push(first);
            new_stones.push(second);
        }

        return self.blink(&new_stones, n - 1);
    }

    fn apply_rule(&self, num: i64, count: i64, state: &mut HashMap<i64, i64>) {
        if num == 0 {
            if state.contains_key(&1) {
                state.insert(1, state[&1] + count);
            } else {
                state.insert(1, count);
            }
            return;
        }

        let (ok, first, second) = self.split_even_len_num(num);
        if ok {
            if state.contains_key(&first) {
                state.insert(first, state[&first] + count);
            } else {
                state.insert(first, count);
            }

            if state.contains_key(&second) {
                state.insert(second, state[&second] + count);
            } else {
                state.insert(second, count);
            }

            return;
        }

        if state.contains_key(&(num * 2024)) {
            state.insert(num * 2024, state[&(num * 2024)] + count);
        } else {
            state.insert(num * 2024, count);
        }
    }

    fn blink_optimized(&self, stones: &Vec<i64>, n: i64) -> i64 {
        let mut curr_state: HashMap<i64, i64> = HashMap::new();

        for stone in stones {
            curr_state.insert(*stone, 1);
        }

        for _ in 0..n {
            let mut new_state: HashMap<i64, i64> = HashMap::new();
            for (stone, count) in curr_state {
                self.apply_rule(stone, count, &mut new_state);
            }

            curr_state = new_state;
        }

        curr_state.values().sum()
    }
}

impl aoclib::Solution for Aoc2024_11 {
    fn id(&self) -> (i32, i32) {
        (2024, 11)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024_11.txt");
        //let lines = aoclib::read_lines("test.txt");

        self.stones = lines[0]
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
    }

    fn part_one(&mut self) -> Vec<String> {
        let stones = self.blink(&self.stones, 25);
        aoclib::output(stones.len())
    }

    fn part_two(&mut self) -> Vec<String> {
        aoclib::output(self.blink_optimized(&self.stones, 75))
    }
}
