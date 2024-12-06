use rustlang::aoc::aoc_2024;
use rustlang::aoclib::*;

fn main() {
    println!("Advent of Code 2024");
    let mut runner = Runner::new(Mode::Last);

    runner.add(Box::new(aoc_2024::Aoc2024_01::new()));
    runner.add(Box::new(aoc_2024::Aoc2024_02::new()));
    runner.add(Box::new(aoc_2024::Aoc2024_03::new()));
    runner.add(Box::new(aoc_2024::Aoc2024_04::new()));
    runner.add(Box::new(aoc_2024::Aoc2024_05::new()));
    runner.add(Box::new(aoc_2024::Aoc2024_06::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_07::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_08::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_09::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_10::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_11::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_12::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_13::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_14::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_15::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_16::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_17::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_18::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_19::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_20::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_21::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_22::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_23::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_24::new()));
    // runner.add(Box::new(aoc_2024::Aoc2024_25::new()));

    runner.run();
}
