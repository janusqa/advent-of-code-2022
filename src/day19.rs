use std::process;
use std::time::Instant;

mod input;
mod part_a;
mod part_b;

pub fn solution() {
    let input = input::get_input("./src/day19/input.txt");
    if let Err(e) = input {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }

    let contents = input.unwrap();

    let now = Instant::now();
    println!("Result: {}", part_a::part_a(&contents));
    println!("Completed in {:.2?}", now.elapsed());
    println!();
    let now = Instant::now();
    println!("Result: {}", part_b::part_b(&contents));
    println!("Completed in {:.2?}", now.elapsed());
}
