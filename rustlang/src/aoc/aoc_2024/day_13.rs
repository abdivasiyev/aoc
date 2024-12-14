use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_13 {
    points: Vec<Vec<Point>>,
}

#[derive(Debug, Default)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Aoc2024_13 {
    pub fn new() -> Self {
        Default::default()
    }

    fn solve_eq(&self, p1: &Point, p2: &Point, p3: &Point) -> Point {
        let lcm = aoclib::lcm(p1.x, p1.y);
        let k1 = lcm / p1.x;
        let k2 = lcm / p1.y;

        let m = (k1 * p3.x - k2 * p3.y) / (k1 * p2.x - k2 * p2.y);
        let n = (p3.x - p2.x * m) / p1.x;

        if p1.x * n + p2.x * m == p3.x && p1.y * n + p2.y * m == p3.y {
            return Point::new(n, m);
        }

        Point::new(0, 0)
    }
}

impl aoclib::Solution for Aoc2024_13 {
    fn id(&self) -> (i32, i32) {
        (2024, 13)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024_13.txt");
        //let lines = aoclib::read_lines("test.txt");
        let mut points: Vec<Point> = vec![];

        for line in lines {
            if line.trim().is_empty() {
                self.points.push(points);
                points = vec![];
                continue;
            }

            let part = line.split_once(": ").unwrap();
            let part = part.1.split_once(", ").unwrap();
            let x;
            let y;

            if !part.0.contains("=") {
                x = part.0.trim_start_matches("X+").parse::<i64>().unwrap();
                y = part.1.trim_start_matches("Y+").parse::<i64>().unwrap();
            } else {
                x = part.0.trim_start_matches("X=").parse::<i64>().unwrap();
                y = part.1.trim_start_matches("Y=").parse::<i64>().unwrap();
            }

            points.push(Point::new(x, y));
        }
        self.points.push(points);
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut total_tokens = 0;

        for points in self.points.iter() {
            let p = self.solve_eq(&points[0], &points[1], &points[2]);

            total_tokens += p.x * 3 + p.y;
        }
        aoclib::output(total_tokens)
    }

    fn part_two(&mut self) -> Vec<String> {
        let mut total_tokens = 0;

        for points in self.points.iter() {
            let p = self.solve_eq(&points[0], &points[1], &Point::new(points[2].x.clone() + 10000000000000, points[2].y + 10000000000000));

            total_tokens += p.x * 3 + p.y;
        }
        aoclib::output(total_tokens)
    }
}
