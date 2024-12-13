use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_12 {
    regions: Vec<Vec<char>>,
}

const DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Aoc2024_12 {
    pub fn new() -> Self {
        Default::default()
    }

    fn is_valid(&self, i: i64, j: i64) -> bool {
        i >= 0 && i < self.regions.len() as i64 && j >= 0 && j < self.regions[0].len() as i64
    }

    fn has_path(&self, i: i64, j: i64, dir: (i64, i64)) -> bool {
        let i2 = i + dir.0;
        let j2 = j + dir.1;

        self.is_valid(i2, j2)
            && self.regions[i as usize][j as usize] == self.regions[i2 as usize][j2 as usize]
    }

    fn dfs(
        &self,
        i: i64,
        j: i64,
        visited: &mut Vec<Vec<bool>>,
        area: &mut i64,
        perimeter: &mut i64,
    ) {
        if !self.is_valid(i, j) {
            return;
        }
        if visited[i as usize][j as usize] {
            return;
        }

        visited[i as usize][j as usize] = true;
        *area += 1;

        for dir in DIRS {
            if self.is_valid(i + dir.0, j + dir.1)
                && self.regions[i as usize][j as usize]
                    == self.regions[(i + dir.0) as usize][(j + dir.1) as usize]
            {
                self.dfs(i + dir.0, j + dir.1, visited, area, perimeter);
            } else {
                *perimeter += 1;
            }
        }
    }

    fn dfs_side(
        &self,
        i: i64,
        j: i64,
        visited: &mut Vec<Vec<bool>>,
        area: &mut i64,
        sides: &mut i64,
    ) {
        if !self.is_valid(i, j) {
            return;
        }
        if visited[i as usize][j as usize] {
            return;
        }

        visited[i as usize][j as usize] = true;
        *area += 1;

        for k in 0..DIRS.len() {
            let dir1 = DIRS[k];
            let dir2 = DIRS[(k + 1) % DIRS.len()];

            if !self.has_path(i, j, dir1) && !self.has_path(i, j, dir2) {
                *sides += 1;
            }

            if self.has_path(i, j, dir1)
                && self.has_path(i, j, dir2)
                && !self.has_path(i, j, (dir1.0 + dir2.0, dir1.1 + dir2.1))
            {
                *sides += 1;
            }
        }

        for dir in DIRS {
            if self.has_path(i, j, dir) && !visited[(i + dir.0) as usize][(j + dir.1) as usize] {
                self.dfs_side(i + dir.0, j + dir.1, visited, area, sides);
            }
        }
    }
}

impl aoclib::Solution for Aoc2024_12 {
    fn id(&self) -> (i32, i32) {
        (2024, 12)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024_12.txt");
        //let lines = aoclib::read_lines("test.txt");

        self.regions = lines.iter().map(|line| line.chars().collect()).collect();
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut visited = vec![vec![false; self.regions[0].len()]; self.regions.len()];
        let mut sum = 0;

        for i in 0..self.regions.len() {
            for j in 0..self.regions[0].len() {
                if visited[i][j] {
                    continue;
                }
                let mut area: i64 = 0;
                let mut perimeter: i64 = 0;
                self.dfs(i as i64, j as i64, &mut visited, &mut area, &mut perimeter);

                sum += area * perimeter;
            }
        }

        aoclib::output(sum)
    }

    fn part_two(&mut self) -> Vec<String> {
        let mut visited = vec![vec![false; self.regions[0].len()]; self.regions.len()];
        let mut sum = 0;

        for i in 0..self.regions.len() {
            for j in 0..self.regions[0].len() {
                if visited[i][j] {
                    continue;
                }
                let mut area: i64 = 0;
                let mut sides: i64 = 0;
                self.dfs_side(i as i64, j as i64, &mut visited, &mut area, &mut sides);

                sum += area * sides;
            }
        }
        aoclib::output(sum)
    }
}
