use crate::aoclib;

#[derive(Default)]
pub struct AocYear_Day {
}

impl AocYear_Day {
    pub fn new() -> Self {
        Default::default()
    }
}

impl aoclib::Solution for AocYear_Day {
    fn id(&self) -> (i32, i32) {
        (year, day)
    }

    fn parse(&mut self) {
    }

    fn part_one(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }

    fn part_two(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
