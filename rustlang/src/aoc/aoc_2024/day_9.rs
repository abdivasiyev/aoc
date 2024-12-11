use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_09 {
    blocks: String,
}

impl Aoc2024_09 {
    pub fn new() -> Self {
        Default::default()
    }
}

impl aoclib::Solution for Aoc2024_09 {
    fn id(&self) -> (i32, i32) {
        (2024, 9)
    }

    fn parse(&mut self) {
        // let lines = aoclib::read_lines("test.txt");
        let lines = aoclib::read_lines("input/2024_9.txt");

        if lines.len() == 0 {
            panic!("No input");
        }

        self.blocks = lines[0].clone();
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut disk: Vec<i64> = self
            .blocks
            .chars()
            .filter(|c| c.is_digit(10))
            .filter_map(|c| c.to_digit(10))
            .map(|n| n as usize)
            .enumerate()
            .flat_map(|(i, n)| {
                if i % 2 == 0 {
                    vec![(i / 2) as i64; n]
                } else {
                    vec![-1 as i64; n]
                }
            })
            .collect();

        let mut i: usize = 0;
        let mut j: usize = disk.len() - 1;

        while i < j {
            if disk[i] != -1 {
                i += 1;
                continue;
            }

            if disk[j] == -1 {
                j -= 1;
                continue;
            }

            disk.swap(i, j);
            i += 1;
            j -= 1;
        }

        let mut sum = 0;

        for i in 0..disk.len() {
            if disk[i] == -1 {
                continue;
            }

            sum += disk[i] as i64 * i as i64;
        }

        aoclib::output(sum)
    }

    fn part_two(&mut self) -> Vec<String> {
        let mut disk: Vec<Block> = self
            .blocks
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|n| n as usize)
            .enumerate()
            .map(|(i, n)| Block::new(i, n))
            .collect();

        let mut j: usize = disk.len() - 1;

        while j > 0 {
            if disk[j].type_ == 1 {
                j -= 1;
                continue;
            }

            let mut i: usize = 0;

            while i < j {
                if disk[i].type_ == 0 {
                    i += 1;
                    continue;
                }

                if disk[i].type_ == 1 && disk[j].data.len() > disk[i].free_space {
                    i += 1;
                    continue;
                }

                let data = disk[j].data.clone();

                disk[i].fill(&data);
                disk[j].clear();
                break;
            }
            j -= 1;
        }

        let sum = disk
            .iter()
            .flat_map(|b| b.data.iter().cloned().chain(std::iter::repeat(0).take(b.free_space)))
            .enumerate()
            .map(|(i, n)| if n != -1 { n * i as i64 } else { 0 })
            .sum::<i64>();

        aoclib::output(sum)
    }
}

#[derive(Debug)]
struct Block {
    type_: usize,
    free_space: usize,
    data: Vec<i64>,
}

impl Block {
    fn new(id: usize, size: usize) -> Self {
        if id % 2 == 0 {
            Self {
                type_: 0,
                free_space: 0,
                data: vec![(id as i64) / 2; size],
            }
        } else {
            Self {
                type_: 1,
                free_space: size,
                data: Vec::new(),
            }
        }
    }

    fn fill(&mut self, data: &Vec<i64>) {
        self.data.extend(data.iter());
        self.free_space -= data.len();
        if self.free_space == 0 {
            self.type_ = 0;
        }
    }

    fn clear(&mut self) {
        self.free_space = self.data.len();
        self.data.clear();
        self.type_ = 1;
    }
}
