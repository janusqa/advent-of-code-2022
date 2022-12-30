use itertools::Itertools;
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    ore: (i32, i32, i32),
    clay: (i32, i32, i32),
    obsidian: (i32, i32, i32),
    geode: (i32, i32, i32),
}

impl Blueprint {
    fn new(
        ore: (i32, i32, i32),
        clay: (i32, i32, i32),
        obsidian: (i32, i32, i32),
        geode: (i32, i32, i32),
    ) -> Blueprint {
        return Blueprint {
            ore,
            clay,
            obsidian,
            geode,
        };
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct StateKey {
    mins: i32,
    ore_robot: i32,
    ore: i32,
    clay_robot: i32,
    clay: i32,
    obsidian_robot: i32,
    obsidian: i32,
    geode_robot: i32,
    geode: i32,
}

impl StateKey {
    fn new(
        mins: i32,
        ore_robot: i32,
        ore: i32,
        clay_robot: i32,
        clay: i32,
        obsidian_robot: i32,
        obsidian: i32,
        geode_robot: i32,
        geode: i32,
    ) -> StateKey {
        return Self {
            mins,
            ore_robot,
            ore,
            clay_robot,
            clay,
            obsidian_robot,
            obsidian,
            geode_robot,
            geode,
        };
    }
}

pub fn part_a(contents: &str) -> i32 {
    lazy_static! {
        static ref RE_BLUEPRINT: Regex = Regex::new(r"^Blueprint (\d+):.+?ore robot costs (\d+).+?clay robot costs (\d+).+?obsidian robot costs (\d+).+?(\d+).+?geode robot costs (\d+).+?(\d+)").unwrap();
    }

    let mut blueprints: HashMap<i32, Blueprint> = HashMap::new();
    for line in contents.lines() {
        let blueprint = RE_BLUEPRINT.captures(line).unwrap();
        blueprints
            .entry(blueprint.get(1).unwrap().as_str().parse::<i32>().unwrap())
            .or_insert(Blueprint::new(
                (
                    blueprint.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    0,
                    0,
                ),
                (
                    blueprint.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                    0,
                    0,
                ),
                (
                    blueprint.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                    blueprint.get(5).unwrap().as_str().parse::<i32>().unwrap(),
                    0,
                ),
                (
                    blueprint.get(6).unwrap().as_str().parse::<i32>().unwrap(),
                    0,
                    blueprint.get(7).unwrap().as_str().parse::<i32>().unwrap(),
                ),
            ));
    }

    let time_limit = 24;
    let inventory = StateKey::new(1, 1, 1, 0, 0, 0, 0, 0, 0);
    let mut result = 0;
    for blueprint in blueprints.keys().sorted_by(|a, b| a.cmp(&b)) {
        // println!(
        //     "Blueprint {}: {:?}",
        //     blueprint,
        //     &blueprints.get(&blueprint).unwrap()
        // );
        let mut memo: HashMap<StateKey, i32> = HashMap::new();
        let max_geodes = dfs(
            &inventory,
            time_limit,
            &blueprints.get(&blueprint).unwrap(),
            &mut memo,
        );
        // println!("Maximum geodes opened: {}", max_geodes);
        // println!();
        result += max_geodes * blueprint;
    }

    return result;
}

fn dfs(
    inventory: &StateKey,
    time_limit: i32,
    blueprint: &Blueprint,
    memo: &mut HashMap<StateKey, i32>,
) -> i32 {
    if memo.contains_key(&inventory) {
        return *memo.get(&inventory).unwrap();
    }

    if inventory.mins >= time_limit {
        return inventory.geode;
    }

    let mut max_geodes = 0;

    // create geode robot
    if inventory.ore >= blueprint.geode.0 && inventory.obsidian >= blueprint.geode.2 {
        return max_geodes.max(dfs(
            &StateKey::new(
                inventory.mins + 1,
                inventory.ore_robot,
                inventory.ore_robot + inventory.ore - blueprint.geode.0,
                inventory.clay_robot,
                inventory.clay_robot + inventory.clay,
                inventory.obsidian_robot,
                inventory.obsidian_robot + inventory.obsidian - blueprint.geode.2,
                inventory.geode_robot + 1,
                inventory.geode_robot + inventory.geode,
            ),
            time_limit,
            blueprint,
            memo,
        ));
    }

    // create obsidian robot
    if inventory.ore >= blueprint.obsidian.0
        && inventory.clay >= blueprint.obsidian.1
        && inventory.obsidian_robot < blueprint.geode.2
    {
        max_geodes = max_geodes.max(dfs(
            &StateKey::new(
                inventory.mins + 1,
                inventory.ore_robot,
                inventory.ore_robot + inventory.ore - blueprint.obsidian.0,
                inventory.clay_robot,
                inventory.clay_robot + inventory.clay - blueprint.obsidian.1,
                inventory.obsidian_robot + 1,
                inventory.obsidian_robot + inventory.obsidian,
                inventory.geode_robot,
                inventory.geode_robot + inventory.geode,
            ),
            time_limit,
            blueprint,
            memo,
        ));
    }

    // create clay robot
    if inventory.ore >= blueprint.clay.0
        && inventory.clay_robot < blueprint.obsidian.1
        && inventory.obsidian_robot < blueprint.geode.2
    {
        max_geodes = max_geodes.max(dfs(
            &StateKey::new(
                inventory.mins + 1,
                inventory.ore_robot,
                inventory.ore_robot + inventory.ore - blueprint.clay.0,
                inventory.clay_robot + 1,
                inventory.clay_robot + inventory.clay,
                inventory.obsidian_robot,
                inventory.obsidian_robot + inventory.obsidian,
                inventory.geode_robot,
                inventory.geode_robot + inventory.geode,
            ),
            time_limit,
            blueprint,
            memo,
        ));
    }

    // create ore robot
    if inventory.ore >= blueprint.ore.0
        && inventory.ore_robot
            < *[
                // blueprint.ore.0,
                blueprint.clay.0,
                blueprint.obsidian.0,
                blueprint.geode.0,
            ]
            .iter()
            .max()
            .unwrap()
    {
        max_geodes = max_geodes.max(dfs(
            &StateKey::new(
                inventory.mins + 1,
                inventory.ore_robot + 1,
                inventory.ore_robot + inventory.ore - blueprint.ore.0,
                inventory.clay_robot,
                inventory.clay_robot + inventory.clay,
                inventory.obsidian_robot,
                inventory.obsidian_robot + inventory.obsidian,
                inventory.geode_robot,
                inventory.geode_robot + inventory.geode,
            ),
            time_limit,
            blueprint,
            memo,
        ));
    }

    // create no robot
    if inventory.ore_robot
        < *[
            // blueprint.ore.0,
            blueprint.clay.0,
            blueprint.obsidian.0,
            blueprint.geode.0,
        ]
        .iter()
        .max()
        .unwrap()
    {
        max_geodes = max_geodes.max(dfs(
            &StateKey::new(
                inventory.mins + 1,
                inventory.ore_robot,
                inventory.ore_robot + inventory.ore,
                inventory.clay_robot,
                inventory.clay_robot + inventory.clay,
                inventory.obsidian_robot,
                inventory.obsidian_robot + inventory.obsidian,
                inventory.geode_robot,
                inventory.geode_robot + inventory.geode,
            ),
            time_limit,
            blueprint,
            memo,
        ));
    }

    memo.entry(inventory.clone()).or_insert(max_geodes);

    return max_geodes;
}
