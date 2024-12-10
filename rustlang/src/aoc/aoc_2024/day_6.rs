use std::collections::HashSet;

use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_06 {
    map: Vec<Vec<char>>,
}

impl Aoc2024_06 {
    pub fn new() -> Self {
        Default::default()
    }
}

impl aoclib::Solution for Aoc2024_06 {
    fn id(&self) -> (i32, i32) {
        (2024, 6)
    }

    fn parse(&mut self) {
        let input = aoclib::read_lines("input/2024_6.txt");
        // let input = aoclib::read_lines("test.txt");

        self.map = input.iter().map(|line| line.chars().collect()).collect();
    }

    fn part_one(&mut self) -> Vec<String> {
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut dir = 0;
        let mut map = self.map.clone();
        let mut curr = (0, 0);
        let mut visited = HashSet::new();

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == '^' {
                    curr = (i as i64, j as i64);
                    map[i][j] = '.';
                }
            }
        }

        loop {
            visited.insert(curr);

            let curr_dir = dirs[dir];
            let i = curr.0 + curr_dir.0;
            let j = curr.1 + curr_dir.1;

            if i < 0 || i >= map.len() as i64 || j < 0 || j >= map[0].len() as i64 {
                break;
            }

            if map[i as usize][j as usize] == '#' {
                dir = (dir + 1) % dirs.len();
                continue;
            }
            curr = (i, j);
        }

        aoclib::output(visited.len())
    }

    fn part_two(&mut self) -> Vec<String> {
        let mut possible_obstacles = 0;
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut map = self.map.clone();

        for obstacle_i in 0..self.map.len() {
            for obstacle_j in 0..self.map[obstacle_i].len() {
                if self.map[obstacle_i][obstacle_j] == '#'
                    || self.map[obstacle_i][obstacle_j] == '^'
                {
                    continue;
                }

                let mut dir = 0;
                let mut curr = (0, 0);
                let mut visited = HashSet::new();
                let mut steps = 0;

                for i in 0..map.len() {
                    for j in 0..map[0].len() {
                        if map[i][j] == '^' {
                            curr = (i as i64, j as i64);
                        }
                    }
                }

                map[obstacle_i][obstacle_j] = '#';

                loop {
                    if steps >= 100_000 {
                        possible_obstacles += 1;
                        break;
                    }
                    steps += 1;
                    visited.insert(curr);

                    let curr_dir = dirs[dir];
                    let i = curr.0 + curr_dir.0;
                    let j = curr.1 + curr_dir.1;

                    if i < 0 || i >= map.len() as i64 || j < 0 || j >= map[0].len() as i64 {
                        break;
                    }

                    if map[i as usize][j as usize] == '#' {
                        dir = (dir + 1) % dirs.len();
                        continue;
                    }
                    curr = (i, j);
                }

                map[obstacle_i][obstacle_j] = '.';
            }
        }

        aoclib::output(possible_obstacles)
    }
}
