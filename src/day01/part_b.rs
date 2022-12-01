use super::input;

pub fn part_b() -> i32 {
    let inventory = match input::get_input("./src/day01/input.txt") {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let mut calories_per_knapsack: Vec<i32> = Vec::new();

    for knapsack in inventory.trim().split("\n\n") {
        let mut calories_in_knapsack = 0;
        for item in knapsack.trim().split("\n") {
            calories_in_knapsack += match item.parse::<i32>() {
                Ok(calories) => calories,
                Err(_) => 0,
            }
        }
        calories_per_knapsack.push(calories_in_knapsack)
    }

    calories_per_knapsack.sort_by(|a, b| b.cmp(a));

    return &calories_per_knapsack[0] + &calories_per_knapsack[1] + &calories_per_knapsack[2];
}
