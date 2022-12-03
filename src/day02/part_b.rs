use std::collections::HashMap;

pub fn part_b(round: &str) -> i32 {
    let mut scores = HashMap::new();
    scores.insert("A", 1);
    scores.insert("B", 2);
    scores.insert("C", 3);

    let mut total_score = 0;

    for turn in round.lines() {
        let choice: Vec<&str> = turn.split(" ").collect();

        let opponent_choice = scores.get(choice[0]).unwrap();
        let opponent_left = (opponent_choice - 1 + 3 - 1) % 3 + 1;
        let opponent_right = (opponent_choice + 1 - 1) % 3 + 1;

        let player_choice = match choice[1] {
            "X" => &opponent_left,
            "Z" => &opponent_right,
            _ => opponent_choice,
        };
        let player_left = (player_choice - 1 + 3 - 1) % 3 + 1;
        let player_right = (player_choice + 1 - 1) % 3 + 1;

        if player_choice == opponent_choice {
            total_score += 3 + player_choice;
        } else if opponent_choice == &player_left {
            total_score += 6 + player_choice
        } else if opponent_choice == &player_right {
            total_score += player_choice
        }
    }

    return total_score;
}
