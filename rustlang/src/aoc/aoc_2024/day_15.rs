use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_15 {
    me: (i64, i64),
    map: Vec<Vec<char>>,
    moves: Vec<(i64, i64)>,
}

const DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Aoc2024_15 {
    pub fn new() -> Self {
        Default::default()
    }

    fn is_movable(&self, map: &Vec<Vec<char>>, me: (i64, i64), m: &(i64, i64)) -> bool {
        let (i, j) = me;
        let (di, dj) = m;

        aoclib::inside((i + di, j + dj), map[0].len() as i64, map.len() as i64)
            && map[(i + di) as usize][(j + dj) as usize] != '#'
    }
}

impl aoclib::Solution for Aoc2024_15 {
    fn id(&self) -> (i32, i32) {
        (2024, 15)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024_15.txt");
        //let lines = aoclib::read_lines("test.txt");

        let mut i = 0;

        while i < lines.len() {
            if lines[i] == "" {
                i += 1;
                break;
            }

            if let Some(j) = lines[i].find(|c| c == '@') {
                self.me = (i as i64, j as i64);
            }

            self.map.push(lines[i].chars().collect());
            i += 1;
        }

        while i < lines.len() {
            lines[i].chars().for_each(|c| match c {
                '<' => self.moves.push(DIRS[2]),
                '>' => self.moves.push(DIRS[0]),
                '^' => self.moves.push(DIRS[3]),
                'v' => self.moves.push(DIRS[1]),
                _ => (),
            });

            i += 1;
        }
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut pos = self.me;
        let moves = self.moves.clone();
        let mut map = self.map.clone();

        for m in moves {
            if !self.is_movable(&map, pos, &m) {
                continue;
            }

            let (mut i, mut j) = (pos.0 + m.0, pos.1 + m.1);
            let mut moveable = true;

            while map[i as usize][j as usize] == 'O' {
                map[i as usize][j as usize] = 'O';
                (i, j) = (i + m.0, j + m.1);

                if !aoclib::inside((i, j), map[0].len() as i64, map.len() as i64) {
                    moveable = false;
                    break;
                }
            }

            if moveable && map[i as usize][j as usize] == '.' {
                map[i as usize][j as usize] = 'O';
                map[pos.0 as usize][pos.1 as usize] = '.';
                pos = (pos.0 + m.0, pos.1 + m.1);
                map[pos.0 as usize][pos.1 as usize] = '@';
            }
        }

        let mut ans = 0;

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == 'O' {
                    ans += 100 * i + j;
                }
            }
        }

        aoclib::output(ans)
    }

    fn part_two(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{}", map[i][j]);
        }
        println!();
    }
}
