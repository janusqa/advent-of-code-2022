use itertools::Itertools;
use std::collections::HashMap;

pub fn part_b(contents: &str) -> i32 {
    let mut elves: Vec<&str> = contents.lines().collect();
    let mut badges: Vec<char> = Vec::new();

    while elves.len() > 0 {
        let mut items: HashMap<char, i32> = HashMap::new();

        // take 3
        let group: Vec<&str> = elves.drain(0..3).collect();

        // identify elf with with biggest backpack
        let rucksack_with_most_items = group.iter().fold(group[0], |acc, &item| {
            if item.len() > acc.len() {
                return item;
            } else {
                return acc;
            }
        });

        // set up this largest backpack to be compared against others
        for c in rucksack_with_most_items.chars() {
            items.entry(c).or_insert(1);
        }

        // identify others in group
        let other_rucksacks: Vec<&str> = group
            .iter()
            .cloned()
            .filter(|member| *member != rucksack_with_most_items)
            .collect();

        // go thur others to look for common item across all rucksacks
        // and add that item to badges
        for rucksack in other_rucksacks {
            for c in rucksack.chars().into_iter().unique() {
                if items.contains_key(&c) {
                    items.insert(c, items.get(&c).unwrap() + 1);
                }
            }
        }

        items.retain(|_k, v| *v == 3);
        badges.push(items.keys().cloned().collect::<Vec<char>>()[0]);
    }

    // calculate the priority of the badges
    let mut total_priorities = 0;
    for badge in badges {
        total_priorities += priority(badge).unwrap_or(0);
    }

    return total_priorities;
}

fn priority(c: char) -> Option<i32> {
    let char_ascii = c as u32;
    if char_ascii >= 65 && char_ascii <= 90 {
        // uppercase
        return Some(char_ascii as i32 - 64 + 26);
    } else if char_ascii >= 97 && char_ascii <= 122 {
        // lowercase
        return Some(char_ascii as i32 - 96);
    } else {
        return None;
    }
}
