use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_10 {
    map: Vec<Vec<char>>,
}

impl Aoc2024_10 {
    pub fn new() -> Self {
        Default::default()
    }

    fn dfs_path_count(&self, map: &mut Vec<Vec<char>>, paths: &mut i64, i: i64, j: i64, curr: i64) -> bool {
        if i < 0 || i >= self.map.len() as i64 || j < 0 || j >= self.map[0].len() as i64 {
            return false;
        }

        if map[i as usize][j as usize] == '9' && curr == 9 {
            *paths += 1;
            return true;
        }

        if curr == 9 {
            return false;
        }


        if map[i as usize][j as usize].to_digit(10).unwrap_or(0) as i64 != curr {
            return false;
        }

        let dirs: Vec<(i64, i64)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        for dir in dirs {
            let x = i as i64 + dir.0;
            let y = j as i64 + dir.1;

            self.dfs_path_count(map, paths, x, y, curr + 1);
        }

        true
    }

    fn dfs(&self, map: &mut Vec<Vec<char>>, i: i64, j: i64, curr: i64) -> bool {
        if i < 0 || i >= self.map.len() as i64 || j < 0 || j >= self.map[0].len() as i64 {
            return false;
        }

        if map[i as usize][j as usize] == '9' && curr == 9 {
            map[i as usize][j as usize] = '-';
            return true;
        }

        if curr == 9 {
            return false;
        }

        if map[i as usize][j as usize].to_digit(10).unwrap_or(0) as i64 != curr {
            return false;
        }

        let dirs: Vec<(i64, i64)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        for dir in dirs {
            let x = i as i64 + dir.0;
            let y = j as i64 + dir.1;

            self.dfs(map, x, y, curr + 1);
        }

        true
    }
}

impl aoclib::Solution for Aoc2024_10 {
    fn id(&self) -> (i32, i32) {
        (2024, 10)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024_10.txt");
        // let lines = aoclib::read_lines("test.txt");

        self.map = lines.iter().map(|line| line.chars().collect()).collect();
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut map = self.map.clone();
        let mut score = 0;

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] != '0' {
                    continue;
                }

                self.dfs(&mut map, i as i64, j as i64, 0);

                let curr_score = map
                    .iter()
                    .flat_map(|row| row.iter().collect::<Vec<_>>())
                    .filter(|c| **c == '-')
                    .count();

                score += curr_score;
                map = self.map.clone();
            }
        }

        aoclib::output(score)
    }

    fn part_two(&mut self) -> Vec<String> {
        let mut map = self.map.clone();
        let mut rating = 0;

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] != '0' {
                    continue;
                }

                let mut paths = 0;

                self.dfs_path_count(&mut map, &mut paths, i as i64, j as i64, 0);

                rating += paths;
                map = self.map.clone();
            }
        }

        aoclib::output(rating)
    }
}
