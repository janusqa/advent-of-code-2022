use std::process;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    const DAY: i32 = 5;

    match DAY {
        1 => day01::solution(),
        2 => day02::solution(),
        3 => day03::solution(),
        4 => day04::solution(),
        5 => day05::solution(),
        _ => process::exit(0),
    }
}
