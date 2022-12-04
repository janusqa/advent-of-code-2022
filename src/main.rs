use std::process;

mod day01;
mod day02;
mod day03;

fn main() {
    const DAY: i32 = 3;

    match DAY {
        1 => day01::solution(),
        2 => day02::solution(),
        3 => day03::solution(),
        _ => process::exit(0),
    }
}
