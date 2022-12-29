use std::process;

mod input;
mod part_a;
mod part_b;

pub fn solution() {
    let input = input::get_input("./src/day18/input.txt");
    if let Err(e) = input {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }

    let contents = input.unwrap();

    println!("{}", part_a::part_a(&contents));
    println!("{}", part_b::part_b(&contents));
}
