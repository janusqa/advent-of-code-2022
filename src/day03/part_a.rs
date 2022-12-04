use std::collections::HashMap;

pub fn part_a(contents: &str) -> i32 {
    let mut shared_items: Vec<char> = Vec::new();

    for rucksack in contents.lines() {
        let mut items: HashMap<char, i32> = HashMap::new();
        let pocket_size = rucksack.len() / 2;
        let (pocket_1, pocket_2) = rucksack.split_at(pocket_size);
        let pocket_1: Vec<char> = pocket_1.chars().collect();
        let pocket_2: Vec<char> = pocket_2.chars().collect();

        let mut pos = 0;
        while pos < pocket_size {
            items.entry(pocket_1[pos]).or_insert(1);
            pos += 1;
        }
        pos = 0;
        while pos < pocket_size {
            if items.contains_key(&pocket_2[pos]) {
                shared_items.push(pocket_2[pos]);
                break;
            }
            pos += 1;
        }
    }

    let mut total_priorities = 0;

    for item in shared_items {
        total_priorities += priority(item).unwrap_or(0);
    }

    return total_priorities;
}

fn priority(c: char) -> Option<i32> {
    let char_ascii = c as u32;
    if char_ascii >= 65 && char_ascii <= 90 {
        return Some(char_ascii as i32 - 64 + 26);
    } else if char_ascii >= 97 && char_ascii <= 122 {
        return Some(char_ascii as i32 - 96);
    } else {
        return None;
    }
}
