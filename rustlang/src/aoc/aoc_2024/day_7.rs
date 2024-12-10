use crate::aoclib;

#[derive(Default)]
pub struct Aoc2024_07 {
    calibrations: Vec<(i64, Vec<i64>)>,
}

impl Aoc2024_07 {
    pub fn new() -> Self {
        Default::default()
    }

    fn is_valid(&self, test_value: i64, nums: Vec<i64>, operators: Vec<String>) -> Option<i64> {
        let n = nums.len() - 1;
        let combinations = aoclib::combinations(&operators, n);

        for combination in combinations {
            let mut val = nums[0];
            let operators = combination.chars().collect::<Vec<char>>();

            for i in 1..nums.len() {
                if val > test_value {
                    break;
                }
                let operator = operators[i - 1];

                match operator {
                    '+' => val += nums[i],
                    '*' => val *= nums[i],
                    '|' => val = format!("{}{}", val, nums[i]).parse::<i64>().unwrap(),
                    _ => panic!("invalid operator"),
                }
            }

            if val == test_value {
                return Some(test_value);
            }
        }

        None
    }
}

impl aoclib::Solution for Aoc2024_07 {
    fn id(&self) -> (i32, i32) {
        (2024, 7)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024_7.txt");
        //let lines = aoclib::read_lines("test.txt");

        for line in lines {
            let line = line.split_once(": ").unwrap();

            match line {
                (test_value, nums) => {
                    let test_value = test_value.parse::<i64>().unwrap();
                    let nums = nums.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();

                    self.calibrations.push((test_value, nums));
                }
            }
        }
    }

    fn part_one(&mut self) -> Vec<String> {
        let calibration_sums = self
            .calibrations
            .iter()
            .map(|(test_value, nums)| {
                self.is_valid(
                    *test_value,
                    nums.to_vec(),
                    vec!["+".to_string(), "*".to_string()],
                )
                .unwrap_or(0)
            })
            .sum::<i64>();
        aoclib::output(calibration_sums)
    }

    fn part_two(&mut self) -> Vec<String> {
        let calibration_sums = self
            .calibrations
            .iter()
            .map(|(test_value, nums)| {
                self.is_valid(
                    *test_value,
                    nums.to_vec(),
                    vec!["+".to_string(), "*".to_string(), "|".to_string()],
                )
                .unwrap_or(0)
            })
            .sum::<i64>();
        aoclib::output(calibration_sums)
    }
}
