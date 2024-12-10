use std::collections::{HashMap, HashSet};

use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_08 {
    indexes: HashMap<char, Vec<(i64, i64)>>,
    map: Vec<Vec<char>>,
    width: i64,
    height: i64,
}

impl Aoc2024_08 {
    pub fn new() -> Self {
        Default::default()
    }
}

impl aoclib::Solution for Aoc2024_08 {
    fn id(&self) -> (i32, i32) {
        (2024, 8)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024_8.txt");
        // let lines = aoclib::read_lines("test.txt");
        let map: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == '.' {
                    continue;
                }

                if !self.indexes.contains_key(&map[i][j]) {
                    self.indexes.insert(map[i][j], Vec::new());
                }

                self.indexes
                    .get_mut(&map[i][j])
                    .unwrap()
                    .push((i as i64, j as i64));
            }
        }

        self.width = map[0].len() as i64;
        self.height = map.len() as i64;
        self.map = map;
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut antinodes: HashSet<(i64, i64)> = HashSet::new();

        for (_, indexes) in self.indexes.iter() {
            for (i, j) in indexes {
                for (k, n) in indexes {
                    if i == k && j == n {
                        continue;
                    }

                    if (k.abs_diff(*i) + n.abs_diff(*j) + 1) < 2 {
                        continue;
                    }

                    let antinode_i: i64 = if i < k {
                        i - k.abs_diff(*i) as i64
                    } else {
                        i + k.abs_diff(*i) as i64
                    };

                    let antinode_j: i64 = if j < n {
                        j - n.abs_diff(*j) as i64
                    } else {
                        j + n.abs_diff(*j) as i64
                    };

                    if antinode_i < 0
                        || antinode_i >= self.height
                        || antinode_j < 0
                        || antinode_j >= self.width
                    {
                        continue;
                    }

                    antinodes.insert((antinode_i, antinode_j));
                }
            }
        }

        aoclib::output(antinodes.len())
    }

    fn part_two(&mut self) -> Vec<String> {
        let mut antinodes: HashSet<(i64, i64)> = HashSet::new();

        for (_, indexes) in self.indexes.iter() {
            if indexes.len() == 1 {
                continue;
            }

            for (i, j) in indexes {
                antinodes.insert((*i, *j));
                for (k, n) in indexes {
                    if i == k && j == n {
                        continue;
                    }

                    let di = if i < k {
                        -1 * k.abs_diff(*i) as i64
                    } else {
                        k.abs_diff(*i) as i64
                    };

                    let dj = if j < n {
                        -1 * n.abs_diff(*j) as i64
                    } else {
                        n.abs_diff(*j) as i64
                    };

                    let mut antinode_i = i + di;
                    let mut antinode_j = j + dj;

                    while antinode_i >= 0
                        && antinode_i < self.height
                        && antinode_j >= 0
                        && antinode_j < self.width
                    {
                        if self.map[antinode_i as usize][antinode_j as usize] != '.' {
                            antinode_i += di;
                            antinode_j += dj;
                            continue;
                        }
                        antinodes.insert((antinode_i, antinode_j));
                        antinode_j += dj;
                        antinode_i += di;
                    }
                }
            }
        }

        aoclib::output(antinodes.len())
    }
}
