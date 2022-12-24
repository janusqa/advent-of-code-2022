use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

// used for memoizing seen states
#[derive(Hash, PartialEq, Eq, Debug)]
struct StateKey {
    valve: String,
    time_remaining: i32,
    opened: Vec<String>,
}

impl StateKey {
    fn new(valve: String, time_remaining: i32, opened: &mut Vec<String>) -> Self {
        return StateKey {
            valve,
            time_remaining,
            opened: opened
                .clone()
                .into_iter()
                .sorted_by(|a, b| a.cmp(&b))
                .collect(),
        };
    }
}

#[derive(Debug)]
struct Valve<'a> {
    _tag: &'a str,
    flow: i32,
    neighbours: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    fn new(tag: &'a str, flow: i32, neighbours: Vec<&'a str>) -> Valve<'a> {
        return Valve {
            _tag: tag,
            flow,
            neighbours,
        };
    }
}

pub fn part_a(contents: &str) -> i32 {
    lazy_static! {
        static ref RE_NODES: Regex = Regex::new(r"\b[A-Z]{2}\b").unwrap();
        static ref RE_NODE_VALUE: Regex = Regex::new(r"\d{1,}").unwrap();
    }

    // Parsing the input
    let mut graph: HashMap<&str, Valve> = HashMap::new();
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
        graph.insert(node_name, Valve::new(node_name, node_value, neighbours));
    }

    // Construct the shortest paths from each node to every other node that has a fuctional valve
    let mut destinations: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    let shortest_paths = build_shortest_paths(&graph);
    for (tag, _) in graph.iter() {
        let selected_destinations = shortest_paths
            .iter()
            .filter(|&(k, _)| {
                (k.0 == *tag && graph.get(&k.1).unwrap().flow > 0)
                    || (k.1 == *tag && graph.get(&k.0).unwrap().flow > 0)
            })
            .fold(HashMap::new(), |mut acc, shortest_path| {
                match *tag == shortest_path.0 .0 {
                    true => {
                        acc.insert(shortest_path.0 .1, *shortest_path.1);
                        acc
                    }
                    false => {
                        acc.insert(shortest_path.0 .0, *shortest_path.1);
                        acc
                    }
                }
            });
        destinations.insert(*tag, selected_destinations);
    }

    // Walking thru the valve system and turning on valves. DFS (Recursion) used.
    let mut opened: Vec<String> = Vec::new();
    let mut memo: HashMap<StateKey, i32> = HashMap::new();
    let time_remaining = 30;
    let mp = max_pressure(
        "AA",
        time_remaining,
        &graph,
        &destinations,
        &mut opened,
        &mut memo,
    );

    return mp;
}

fn max_pressure(
    valve: &str,
    time_remaining: i32,
    graph: &HashMap<&str, Valve>,
    destinations: &HashMap<&str, HashMap<&str, i32>>,
    opened: &mut Vec<String>,
    memo: &mut HashMap<StateKey, i32>,
) -> i32 {
    let key = StateKey::new(String::from(valve), time_remaining, opened);

    if let Some(cached_pressure) = memo.get(&key) {
        return *cached_pressure;
    }

    if time_remaining <= 0 {
        return 0;
    }

    let mut mp = 0;

    if graph.get(&valve).unwrap().flow > 0 && !opened.contains(&String::from(valve)) {
        opened.push(String::from(valve));
        mp = mp.max(
            (graph.get(&valve).unwrap().flow * (time_remaining - 1))
                + max_pressure(valve, time_remaining - 1, graph, destinations, opened, memo),
        );
        opened.pop();
    }

    for (destination, time_elapsed) in destinations.get(&valve).unwrap().iter() {
        if !opened.contains(&String::from(*destination)) {
            mp = mp.max(max_pressure(
                *destination,
                time_remaining - *time_elapsed,
                graph,
                destinations,
                opened,
                memo,
            ));
        }
    }

    memo.entry(key).or_insert(mp);

    return mp;
}

fn build_shortest_paths<'a>(graph: &HashMap<&'a str, Valve>) -> HashMap<(&'a str, &'a str), i32> {
    let mut shortest_paths: HashMap<(&str, &str), i32> = HashMap::new();
    for start in graph.keys() {
        for end in graph.keys() {
            if *start == *end {
                continue;
            }

            if shortest_paths.contains_key(&(*end, *start)) {
                continue;
            }

            shortest_paths.insert((*start, *end), shortest_path(*start, *end, graph));
        }
    }

    return shortest_paths;
}

fn shortest_path(start: &str, end: &str, graph: &HashMap<&str, Valve>) -> i32 {
    let mut visited: HashSet<&str> = HashSet::new();

    let mut explore: VecDeque<(&str, i32)> = VecDeque::from([(start, 0)]);
    visited.insert(start);

    while explore.len() > 0 {
        let (current, distance) = explore.pop_front().unwrap();

        if current == end {
            return distance;
        }

        for neighbour in &graph.get(&current).unwrap().neighbours {
            if visited.contains(&*neighbour) {
                continue;
            }

            explore.push_back((*neighbour, distance + 1));

            visited.insert(*neighbour);
        }
    }

    return 0;
}
