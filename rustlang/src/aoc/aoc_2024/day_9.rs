use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_09 {
    blocks: Vec<i64>,
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

        let mut id: i64 = 0;

        for (i, c) in lines[0].chars().enumerate() {
            let num = c.to_digit(10).unwrap();

            if i % 2 == 0 {
                self.blocks.extend((0..num).map(|_| id));
                id += 1;
            } else {
                self.blocks
                    .extend((0..num).map(|_| -1).collect::<Vec<i64>>());
            }
        }
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut i: usize = 0;
        let mut j: usize = self.blocks.len() - 1;
        let mut blocks = self.blocks.clone();

        while i < j {
            if blocks[i] != -1 {
                i += 1;
                continue;
            }

            if blocks[j] == -1 {
                j -= 1;
                continue;
            }

            blocks.swap(i, j);
            i += 1;
            j -= 1;
        }

        let mut sum = 0;

        for i in 0..blocks.len() {
            if blocks[i] == -1 {
                continue;
            }

            sum += blocks[i] as i64 * i as i64;
        }

        aoclib::output(sum)
    }

    fn part_two(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
