use std::collections::HashMap;

pub fn part_a(round: &str) -> i32 {
    let mut scores = HashMap::new();
    scores.insert("A", 1);
    scores.insert("B", 2);
    scores.insert("C", 3);
    scores.insert("X", 1);
    scores.insert("Y", 2);
    scores.insert("Z", 3);

    let mut total_score = 0;

    for turn in round.lines() {
        let choice: Vec<&str> = turn.split(" ").collect();
        let opponent_choice = scores.get(choice[0]).unwrap();
        let player_choice = scores.get(choice[1]).unwrap();
        let left = (player_choice - 1 + 3 - 1) % 3 + 1;
        let right = (player_choice + 1 - 1) % 3 + 1;

        if player_choice == opponent_choice {
            total_score += 3 + player_choice;
        } else if opponent_choice == &left {
            total_score += 6 + player_choice
        } else if opponent_choice == &right {
            total_score += player_choice
        }
    }

    return total_score;
}
