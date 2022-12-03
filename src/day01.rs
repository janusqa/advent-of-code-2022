mod input;
mod part_a;
mod part_b;

pub fn solution() {
    println!("Maximum calories carried: {}", part_a::part_a());
    println!("Combined calories of Top 3: {}", part_b::part_b());
}
