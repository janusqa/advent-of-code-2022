use std::process;

mod day01;
mod day02;

fn main() {
    const DAY: i32 = 2;

    match DAY {
        1 => day01::solution(),
        2 => day02::solution(),
        _ => process::exit(0),
    }
}
