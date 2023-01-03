use std::collections::HashMap;

pub fn part_b(contents: &str) -> i64 {
    let mut monkeys: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut humn: Vec<(String, i64, &str)> = Vec::new();

    for line in contents.lines() {
        let monkey_node = line.split(": ").collect::<Vec<&str>>();

        monkeys
            .entry(monkey_node[0])
            .or_insert(monkey_node[1].split(" ").collect::<Vec<&str>>());
    }

    if let Some(result) = monkey_business("root", monkeys.get("root").unwrap(), &monkeys, &mut humn)
    {
        return result;
    } else {
        return -1;
    }
}

fn monkey_business(
    node_name: &str,
    node: &Vec<&str>,

    monkeys: &HashMap<&str, Vec<&str>>,
    humn: &mut Vec<(String, i64, &str)>,
) -> Option<i64> {
    if node_name == "humn" {
        return None;
    }

    if let Ok(num) = node[0].parse::<i64>() {
        return Some(num);
    };

    let mut result;

    let left = monkey_business(node[0], monkeys.get(&node[0]).unwrap(), monkeys, humn);
    let right = monkey_business(node[2], monkeys.get(&node[2]).unwrap(), monkeys, humn);

    if let Some(left_monkey) = left {
        if let Some(right_monkey) = right {
            match node[1] {
                "+" => {
                    result = Some(left_monkey + right_monkey);
                }
                "-" => {
                    result = Some(left_monkey - right_monkey);
                }
                "*" => {
                    result = Some(left_monkey * right_monkey);
                }
                _ => {
                    result = Some(left_monkey / right_monkey);
                }
            }
        } else {
            if let Some(left_monkey) = left {
                humn.push((node[1].to_string(), left_monkey, "eval_right"));
            }
            result = None;
        }
    } else {
        if let Some(right_monkey) = right {
            humn.push((node[1].to_string(), right_monkey, "eval_left"));
        }
        result = None;
    }

    if node_name == "root" {
        let mut resolved_side: i64 = humn.pop().unwrap().1;

        while humn.len() > 0 {
            let evaluated = humn.pop().unwrap();
            match evaluated.0.as_str() {
                "+" => resolved_side -= evaluated.1,
                "-" => match evaluated.2 {
                    "eval_left" => resolved_side += evaluated.1,
                    _ => resolved_side = evaluated.1 - resolved_side,
                },
                "*" => resolved_side /= evaluated.1,
                _ => match evaluated.2 {
                    "eval_left" => resolved_side *= evaluated.1,
                    _ => resolved_side = evaluated.1 / resolved_side,
                },
            }
        }
        result = Some(resolved_side);
    }

    return result;
}
