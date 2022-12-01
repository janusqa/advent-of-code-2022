use super::input;

pub fn part_a() -> i32 {
    let inventory = match input::get_input("./src/day01/input.txt") {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let mut max_calories = 0;

    for knapsack in inventory.trim().split("\n\n") {
        let mut calories_in_knapsack = 0;
        for item in knapsack.trim().split("\n") {
            calories_in_knapsack += match item.parse::<i32>() {
                Ok(calories) => calories,
                Err(_) => 0,
            }
        }
        if calories_in_knapsack > max_calories {
            max_calories = calories_in_knapsack;
        }
    }
    return max_calories;
}
