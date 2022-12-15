extern crate queues;

use lazy_static::lazy_static;
use queues::*;
use regex::Regex;

pub fn part_a(contents: &str) -> i32 {
    lazy_static! {
        static ref RE_ITEMS: Regex = Regex::new(r"\d{1,}").unwrap();
        static ref RE_OPERATION: Regex = Regex::new(r"(.) (\d{1,}|old)$").unwrap();
        static ref RE_TEST: Regex = Regex::new(r"\d{1,}$").unwrap();
        static ref RE_TEST_AND_MATES: Regex = Regex::new(r"\d{1,}$").unwrap();
    }

    let mut monkeys = contents
        .trim()
        .split("\n\n")
        .map(|x| {
            let mut monkey = (queue![], " ", 0, 0, 0, 0, 0);
            let mut monkey_stat = x.trim().split("\n");

            monkey_stat.next();

            let mut current_stat = monkey_stat.next().unwrap();
            for item in RE_ITEMS.find_iter(current_stat) {
                monkey
                    .0
                    .add(item.as_str().parse::<usize>().unwrap())
                    .unwrap();
            }

            current_stat = monkey_stat.next().unwrap();
            let cap = RE_OPERATION.captures(current_stat);
            monkey.1 = cap.as_ref().unwrap().get(1).unwrap().as_str();
            monkey.2 = cap
                .as_ref()
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap_or(0);

            current_stat = monkey_stat.next().unwrap();
            monkey.3 = RE_TEST
                .find(current_stat)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            current_stat = monkey_stat.next().unwrap();
            monkey.4 = RE_TEST
                .find(current_stat)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            current_stat = monkey_stat.next().unwrap();
            monkey.5 = RE_TEST
                .find(current_stat)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            return monkey;
        })
        .collect::<Vec<(Queue<usize>, &str, usize, usize, usize, usize, usize)>>();

    // println!("{:?}", monkeys);

    for _ in 0..20 {
        for index in 0..monkeys.len() {
            if monkeys[index].0.size() > 0 {
                let success_monkey = monkeys[index].4;
                let fail_monkey = monkeys[index].5;

                while monkeys[index].0.size() > 0 {
                    let mut worry_level = monkeys[index].0.remove().unwrap();

                    let operation = match monkeys[index].2 {
                        0 => worry_level,
                        _ => monkeys[index].2,
                    };

                    worry_level = match monkeys[index].1 {
                        "*" => worry_level * operation,
                        "+" => worry_level + operation,
                        _ => worry_level,
                    };

                    worry_level /= 3;

                    if worry_level % monkeys[index].3 == 0 {
                        monkeys[success_monkey].0.add(worry_level).unwrap();
                    } else {
                        monkeys[fail_monkey].0.add(worry_level).unwrap();
                    }
                    monkeys[index].6 += 1;
                }
            }
        }
    }

    // println!("{:?}", monkeys);
    monkeys.sort_by(|a, b| b.6.cmp(&a.6));

    return (monkeys[0].6 * monkeys[1].6) as i32;
}
