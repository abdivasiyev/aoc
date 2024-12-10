use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_06 {
    map: Vec<Vec<char>>,
}

impl Aoc2024_06 {
    pub fn new() -> Self {
        Default::default()
    }

    fn dfs_part_one(&self, map: &mut Vec<Vec<char>>, i: i64, j: i64, dir: (i64, i64)) -> bool {
        if i < 0 || i >= self.map.len() as i64 || j < 0 || j >= self.map[0].len() as i64 {
            return false;
        }

        if map[i as usize][j as usize] == '#' {
            return false;
        }

        map[i as usize][j as usize] = 'X';

        let (new_i, new_j) = (i + dir.0, j + dir.1);

        if new_i < 0
            || new_i >= self.map.len() as i64
            || new_j < 0
            || new_j >= self.map[0].len() as i64
        {
            return true;
        }

        let mut new_dir = dir;

        if self.map[new_i as usize][new_j as usize] == '#' {
            new_dir = match dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => return false,
            };
        }

        self.dfs_part_one(map, i + new_dir.0, j + new_dir.1, new_dir)
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
        let mut count = 0;
        let mut map = self.map.clone();

        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j] == '^' {
                    self.dfs_part_one(&mut map, i as i64, j as i64, (-1, 0));
                }
            }
        }

        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if map[i][j] == 'X' {
                    count += 1;
                }
            }
        }

        aoclib::output(count)
    }

    fn part_two(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
