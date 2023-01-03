use std::collections::HashMap;

pub fn part_a(contents: &str) -> i64 {
    let mut monkeys: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in contents.lines() {
        let monkey_node = line.split(": ").collect::<Vec<&str>>();

        monkeys
            .entry(monkey_node[0])
            .or_insert(monkey_node[1].split(" ").collect::<Vec<&str>>());
    }

    let result = monkey_business(monkeys.get("root").unwrap(), &monkeys);

    return result;
}

// Binary Tree DFS using post-order traversal (visit left child, then right child and finally the parent)
fn monkey_business(node: &Vec<&str>, monkeys: &HashMap<&str, Vec<&str>>) -> i64 {
    if let Ok(num) = node[0].parse::<i64>() {
        return num;
    };

    let left_monkey = monkey_business(monkeys.get(&node[0]).unwrap(), monkeys);
    let right_monkey = monkey_business(monkeys.get(&node[2]).unwrap(), monkeys);
    match node[1] {
        "+" => {
            return left_monkey + right_monkey;
        }
        "-" => {
            return left_monkey - right_monkey;
        }
        "*" => {
            return left_monkey * right_monkey;
        }
        _ => {
            return left_monkey / right_monkey;
        }
    }
}
