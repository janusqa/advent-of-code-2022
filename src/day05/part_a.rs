use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub fn part_a(contents: &str) -> String {
    let mut stacks: HashMap<usize, Vec<&str>> = HashMap::new();
    let mut stacks_columns: HashMap<usize, usize> = HashMap::new();

    lazy_static! {
        static ref RE_INSTRUCTIONS: Regex = Regex::new(r"(\d{1,})").unwrap();
        static ref RE_STACKS_DATA: Regex = Regex::new(r"\[([A-Z])\]").unwrap();
        static ref RE_STACKS_COLUMNS_DATA: Regex = Regex::new(r"^ \d").unwrap();
        static ref RE_STACKS_COLUMNS: Regex = Regex::new(r"\d{1,}").unwrap();
    }

    // 1. parse stacks columnn names
    let stacks_columns_data = contents
        .lines()
        .filter(|line| RE_STACKS_COLUMNS_DATA.is_match(line))
        .map(|line| {
            RE_STACKS_COLUMNS
                .find_iter(line)
                .map(|mat| (mat.start() - 1, mat.as_str().parse::<usize>().unwrap()))
        })
        .flatten()
        .collect::<Vec<_>>();

    for col in stacks_columns_data {
        stacks_columns.insert(col.0, col.1);
    }
    // println!("{:?}", stacks_columns);

    // 2. parse stacks data
    let stacks_data = contents
        .lines()
        .filter(|line| RE_STACKS_DATA.is_match(line))
        .map(|line| {
            RE_STACKS_DATA
                .find_iter(line)
                .map(|mat| (mat.start(), mat.as_str()))
                .collect::<Vec<(usize, &str)>>()
        })
        .collect::<Vec<_>>();

    for data in stacks_data.iter().rev() {
        for item in data {
            let key = stacks_columns.get(&item.0).unwrap();
            if stacks.contains_key(key) {
                stacks.get_mut(key).unwrap().push(item.1);
            } else {
                stacks.insert(*key, vec![item.1]);
            }
        }
    }
    // println!("{:?}", stacks);

    // 3. parse rearrangement procedure
    let instructions = contents
        .lines()
        .filter(|line| line.contains("move"))
        .map(|line| {
            let matches = RE_INSTRUCTIONS
                .find_iter(line)
                .map(|mat| mat.as_str())
                .map(|mat| mat.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (matches[0], matches[1], matches[2])
        })
        .collect::<Vec<(usize, usize, usize)>>();
    // println!("{:?}", instructions);

    // 4. Perform procedure
    for instruction in instructions {
        let mut num_items_to_move = instruction.0;

        while num_items_to_move > 0 {
            let item = stacks.get_mut(&instruction.1).unwrap().pop().unwrap();

            stacks.get_mut(&instruction.2).unwrap().push(item);
            num_items_to_move -= 1;
        }
    }

    // 5. get top crate from each stack
    let mut sorted = stacks.into_iter().collect::<Vec<(usize, Vec<&str>)>>();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    // println!("{:?}", sorted);

    let crates = sorted.iter().fold(String::new(), |mut acc, item| {
        acc.push(item.1[item.1.len() - 1].chars().nth(1).unwrap());
        acc
    });

    return crates;
}
