use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_04 {
    table: Vec<Vec<char>>,
}

impl Aoc2024_04 {
    pub fn new() -> Self {
        Default::default()
    }
}

impl aoclib::Solution for Aoc2024_04 {
    fn id(&self) -> (i32, i32) {
        (2024, 4)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024_4.txt");
        // let lines = aoclib::read_lines("test.txt");

        for line in lines {
            self.table.push(line.chars().collect());
        }
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut count = 0;

        let directions: [(i64, i64); 8] = [
            // right
            (0, 1),
            // left
            (0, -1),
            // up
            (-1, 0),
            // down
            (1, 0),
            // diagonal right down
            (1, 1),
            // diagonal left down
            (1, -1),
            // diagonal right up
            (-1, 1),
            // diagonal left up
            (-1, -1),
        ];

        let word = ['M', 'A', 'S'];

        for i in 0..self.table.len() {
            for j in 0..self.table[i].len() {
                if self.table[i][j] != 'X' {
                    continue;
                }

                for direction in directions {
                    let mut found = true;

                    for k in 1..=word.len() {
                        let x = i as i64 + direction.0 * k as i64;
                        let y = j as i64 + direction.1 * k as i64;

                        if x < 0 || x >= self.table.len() as i64 || y < 0 || y >= self.table[i].len() as i64 {
                            found = false;
                            break;
                        }

                        if self.table[x as usize][y as usize] != word[k - 1] {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        count += 1;
                    }
                }
            }
        }

        aoclib::output(count)
    }

    fn part_two(&mut self) -> Vec<String> {
        let mut count = 0;

        for i in 0..self.table.len() {
            for j in 0..self.table[i].len() {
                if self.table[i][j] != 'A' {
                    continue;
                }

                if i as i64 - 1 < 0 || i + 1 >= self.table.len() {
                    continue;
                }

                if j as i64 - 1 < 0 || j + 1 >= self.table[i].len() {
                    continue;
                }

                if self.table[i - 1][j - 1] == self.table[i + 1][j + 1] {
                    continue;
                }

                if self.table[i - 1][j + 1] == self.table[i + 1][j - 1] {
                    continue;
                }

                let borders = [
                    self.table[i - 1][j - 1],
                    self.table[i - 1][j + 1],
                    self.table[i + 1][j - 1],
                    self.table[i + 1][j + 1],
                ];

                if borders.iter().filter(|&x| x == &'M' || x == &'S').count() < 4 {
                    continue;
                }
                count += 1;
            }
        }

        aoclib::output(count)
    }
}
