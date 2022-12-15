extern crate queues;

use lazy_static::lazy_static;
use queues::*;
use regex::Regex;

pub fn part_b(contents: &str) -> i64 {
    lazy_static! {
        static ref RE_ITEMS: Regex = Regex::new(r"\d{1,}").unwrap();
        static ref RE_OPERATION: Regex = Regex::new(r"(.) (\d{1,}|old)$").unwrap();
        static ref RE_TEST: Regex = Regex::new(r"\d{1,}$").unwrap();
        static ref RE_TEST_AND_MATES: Regex = Regex::new(r"\d{1,}$").unwrap();
    }

    let mut worry_modifier = 1;

    enum Operation {
        Add(usize),
        Mul(usize),
    }

    impl Operation {
        fn exec(&self, worry_level: usize) -> usize {
            match self {
                Self::Add(n) => match n {
                    0 => worry_level + worry_level,
                    _ => worry_level + n,
                },
                Self::Mul(n) => match n {
                    0 => worry_level * worry_level,
                    _ => worry_level * n,
                },
            }
        }
    }

    let mut monkeys = contents
        .trim()
        .split("\n\n")
        .map(|x| {
            let mut monkey = (queue![], " ", Operation::Add(0), 0, 0, 0, 0);
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
            let operand = cap
                .as_ref()
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap_or(0);
            monkey.2 = match monkey.1 {
                "+" => Operation::Add(operand),
                _ => Operation::Mul(operand),
            };
            current_stat = monkey_stat.next().unwrap();
            monkey.3 = RE_TEST
                .find(current_stat)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            worry_modifier *= monkey.3;

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
        .collect::<Vec<(Queue<usize>, &str, Operation, usize, usize, usize, usize)>>();

    for _ in 0..10000 {
        for index in 0..monkeys.len() {
            if monkeys[index].0.size() > 0 {
                while monkeys[index].0.size() > 0 {
                    let mut worry_level = monkeys[index].0.remove().unwrap();

                    worry_level = monkeys[index].2.exec(worry_level);

                    worry_level %= worry_modifier;

                    let monkey_buddy = if worry_level % monkeys[index].3 == 0 {
                        monkeys[index].4
                    } else {
                        monkeys[index].5
                    };
                    monkeys[monkey_buddy].0.add(worry_level).unwrap();
                    monkeys[index].6 += 1;
                }
            }
        }
        // println!(" Round {}\n {:?}", i + 1, monkeys);
    }

    monkeys.sort_by(|a, b| b.6.cmp(&a.6));

    // NEEDED TO CHANGE TO I64 TO PREVENT OVERFLOW!!!!!!!!
    return (monkeys[0].6 * monkeys[1].6) as i64;
}
