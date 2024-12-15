use std::{os::unix::thread, thread::sleep};

use crate::aoclib;

const W: i64 = 101;
const H: i64 = 103;
const TICKS: i64 = 100;
const COORDS: [(i64, i64, i64, i64); 4] = [
    (0, 0, W / 2 - 1, H / 2 - 1),
    (W / 2 + 1, 0, W, H / 2 - 1),
    (0, H / 2 + 1, W / 2 - 1, H),
    (W / 2 + 1, H / 2 + 1, W, H),
];

#[derive(Default)]
pub struct Aoc2024_14 {
    robots: Vec<Robot>,
}

#[derive(Debug, Default, Clone)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    fn new(pos: (i64, i64), vel: (i64, i64)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            vx: vel.0,
            vy: vel.1,
        }
    }

    fn tick(&mut self, w: i64, h: i64, tick: i64) {
        for _ in 0..tick {
            self.x = (self.x + self.vx + w) % w;
            self.y = (self.y + self.vy + h) % h;
        }
    }

    fn inside(&self, start_x: i64, start_y: i64, end_x: i64, end_y: i64) -> bool {
        self.x >= start_x && self.x <= end_x && self.y >= start_y && self.y <= end_y
    }
}

impl Aoc2024_14 {
    pub fn new() -> Self {
        Default::default()
    }
}

impl aoclib::Solution for Aoc2024_14 {
    fn id(&self) -> (i32, i32) {
        (2024, 14)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024_14.txt");
        // let lines = aoclib::read_lines("test.txt");

        for line in lines {
            let parts = line.split_once(" ").unwrap();
            let pos = parts.0.trim_start_matches("p=").split_once(",").unwrap();
            let vel = parts.1.trim_start_matches("v=").split_once(",").unwrap();

            let pos = (pos.0.parse::<i64>().unwrap(), pos.1.parse::<i64>().unwrap());
            let vel = (vel.0.parse::<i64>().unwrap(), vel.1.parse::<i64>().unwrap());

            self.robots.push(Robot::new(pos, vel));
        }
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut robots = self.robots.clone();
        let mut quadrants = [0; 4];

        for i in 0..robots.len() {
            robots[i].tick(W, H, TICKS);

            for (j, coord) in COORDS.iter().enumerate() {
                if robots[i].inside(coord.0, coord.1, coord.2, coord.3) {
                    quadrants[j] += 1;
                }
            }
        }

        aoclib::output(quadrants.iter().fold(1, |acc, x| acc * x))
    }

    fn part_two(&mut self) -> Vec<String> {

        // y=101*x+11
        // y=103*x+89
        //
        // x=-39
        // y=-3928
        // ticks = y + W*H = -3928+101*103=6475

        // vertical: 11 ticks
        // horizontal: 89 ticks
        let ticks = 6475;
        let mut robots = self.robots.clone();
        let mut map = vec![vec![false; W as usize]; H as usize];

        for i in 0..robots.len() {
            robots[i].tick(W, H, ticks);
            map[robots[i].y as usize][robots[i].x as usize] = true;
        }

        map.iter().for_each(|row| {
            row.iter().for_each(|cell| {
                print!("{}", if *cell { "*" } else { " " });
            });
            println!();
        });

        aoclib::output(ticks)
    }
}
