use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub fn part_a(contents: &str) -> i32 {
    lazy_static! {
        static ref RE_NODES: Regex = Regex::new(r"\b[A-Z]{2}\b").unwrap();
        static ref RE_NODE_VALUE: Regex = Regex::new(r"\d{1,}").unwrap();
    }

    let mut graph: HashMap<&str, (i32, Vec<&str>)> = HashMap::new();
    for line in contents.lines() {
        let mut nodes = RE_NODES.find_iter(line);
        let node_value = RE_NODE_VALUE
            .find(line)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let node_name = nodes.next().unwrap().as_str();
        let mut neighbours: Vec<&str> = Vec::new();
        while let Some(neighbour) = nodes.next() {
            neighbours.push(neighbour.as_str());
        }
        graph.insert(node_name, (node_value, neighbours));
    }

    let mut opened: Vec<String> = Vec::new();
    let mut memo: HashMap<String, i32> = HashMap::new();
    let time_remaining = 30;
    let mp = max_pressure("AA", &graph, &mut opened, time_remaining, &mut memo);

    return mp;
}

fn max_pressure(
    valve: &str,
    graph: &HashMap<&str, (i32, Vec<&str>)>,
    opened: &mut Vec<String>,
    time_remaining: i32,
    memo: &mut HashMap<String, i32>,
) -> i32 {
    let key = format!(
        "{}{}{}",
        valve,
        opened
            .iter()
            .sorted_by(|a, b| a.cmp(&b))
            .fold(String::new(), |acc, v| format!("{}{}", acc, v)),
        time_remaining
    );

    if memo.contains_key(&key) {
        return *(memo.get(&key).unwrap());
    }

    if time_remaining <= 0 {
        return 0;
    }

    let mut mp = 0;

    for neighbour in &graph.get(valve).unwrap().1 {
        mp = mp.max(max_pressure(
            neighbour,
            graph,
            opened,
            time_remaining - 1,
            memo,
        ));

        if graph.get(valve).unwrap().0 > 0 && !opened.contains(&String::from(valve)) {
            opened.push(String::from(valve));
            mp = mp.max(
                (graph.get(valve).unwrap().0 * (time_remaining - 1))
                    + max_pressure(neighbour, graph, opened, time_remaining - 2, memo),
            );
            opened.pop();
        }

        memo.entry(key.clone()).or_insert(mp);
    }

    return mp;
}
