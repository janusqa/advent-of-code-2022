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
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

fn main() {
    const DAY: i32 = 19;

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
        13 => day13::solution(),
        14 => day14::solution(),
        15 => day15::solution(),
        16 => day16::solution(),
        17 => day17::solution(),
        18 => day18::solution(),
        19 => day19::solution(),
        _ => process::exit(0),
    }
}
