use rustlang::aoc::aoc_2023;
use rustlang::aoclib::*;

fn main() {
    let mut runner = Runner::new(Mode::Last);

    runner.add(Box::new(aoc_2023::Aoc2023_01::new()));

    runner.run();
}
