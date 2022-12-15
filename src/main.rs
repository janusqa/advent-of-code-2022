use std::process;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() {
    const DAY: i32 = 12;

    match DAY {
        1 => day01::solution(),
        2 => day02::solution(),
        3 => day03::solution(),
        4 => day04::solution(),
        5 => day05::solution(),
        6 => day06::solution(),
        7 => day07::solution(),
        8 => day08::solution(),
        9 => day09::solution(),
        10 => day10::solution(),
        11 => day11::solution(),
        12 => day12::solution(),
        _ => process::exit(0),
    }
}
