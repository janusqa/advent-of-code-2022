use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
enum Facing {
    Up,
    Right,
    Down,
    Left,
}

impl Facing {
    fn turn(&self, instruction: &str) -> Facing {
        match self {
            Facing::Up => match instruction {
                "R" => Facing::Right,
                "L" => Facing::Left,
                _ => *self,
            },
            Facing::Right => match instruction {
                "R" => Facing::Down,
                "L" => Facing::Up,
                _ => *self,
            },
            Facing::Down => match instruction {
                "R" => Facing::Left,
                "L" => Facing::Right,
                _ => *self,
            },
            Facing::Left => match instruction {
                "R" => Facing::Up,
                "L" => Facing::Down,
                _ => *self,
            },
        }
    }

    fn step(&self) -> (i32, i32) {
        match self {
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
        }
    }

    fn constant(&self) -> i32 {
        return match self {
            Facing::Up => 3,
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
        };
    }
}

#[derive(Debug)]
enum Instruction<'a> {
    Step(i32),
    Turn(&'a str),
}

impl<'a> Instruction<'a> {
    fn translate(&self, state: &State, map: &HashMap<(i32, i32), char>) -> State {
        match self {
            Instruction::Step(s) => {
                let (mut row, mut col) = state.pos;
                for _ in 0..*s {
                    let r = row + state.facing.step().0;
                    let c = col + state.facing.step().1;
                    if map.contains_key(&(r, c)) {
                        if *map.get(&(r, c)).unwrap() == '.' {
                            row = r;
                            col = c;
                        } else {
                            // we have hit a wall
                            break;
                        }
                    } else {
                        // we need to wrap around, as we are in dead space
                        match state.facing {
                            Facing::Up => {
                                let wrap_pos = map
                                    .iter()
                                    .filter(|&(key, _)| key.1 == col)
                                    .sorted_by(|a, b| a.0 .0.cmp(&b.0 .0))
                                    .last()
                                    .unwrap();
                                if *wrap_pos.1 == '#' {
                                    break;
                                }
                                row = wrap_pos.0 .0;
                                col = wrap_pos.0 .1;
                            }
                            Facing::Right => {
                                let wrap_pos = map
                                    .iter()
                                    .filter(|&(key, _)| key.0 == row)
                                    .sorted_by(|a, b| b.0 .1.cmp(&a.0 .1))
                                    .last()
                                    .unwrap();
                                if *wrap_pos.1 == '#' {
                                    break;
                                }
                                row = wrap_pos.0 .0;
                                col = wrap_pos.0 .1;
                            }
                            Facing::Down => {
                                let wrap_pos = map
                                    .iter()
                                    .filter(|&(key, _)| key.1 == col)
                                    .sorted_by(|a, b| b.0 .0.cmp(&a.0 .0))
                                    .last()
                                    .unwrap();
                                if *wrap_pos.1 == '#' {
                                    break;
                                }
                                row = wrap_pos.0 .0;
                                col = wrap_pos.0 .1;
                            }
                            Facing::Left => {
                                let wrap_pos = map
                                    .iter()
                                    .filter(|&(key, _)| key.0 == row)
                                    .sorted_by(|a, b| a.0 .1.cmp(&b.0 .1))
                                    .last()
                                    .unwrap();
                                if *wrap_pos.1 == '#' {
                                    break;
                                }
                                row = wrap_pos.0 .0;
                                col = wrap_pos.0 .1;
                            }
                        }
                    }
                }
                let mut new_state = state.clone();
                new_state.pos.0 = row;
                new_state.pos.1 = col;
                return new_state;
            }
            Instruction::Turn(t) => {
                let mut new_state = state.clone();
                new_state.facing = state.facing.turn(*t);
                return new_state;
            }
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    pos: (i32, i32),
    facing: Facing,
}

impl State {
    fn new(pos: (i32, i32), facing: Facing) -> State {
        return State { pos, facing };
    }
}

pub fn part_a(contents: &str) -> i32 {
    lazy_static! {
        static ref RE_INSTRUCTION_STEPS: Regex = Regex::new(r"(\d{1,})").unwrap();
        static ref RE_INSTRUCTION_TURNS: Regex = Regex::new(r"[RL]").unwrap();
    }

    // parse instructions
    let mut instructions: Vec<Instruction> = Vec::new();
    let instructions_raw = contents.lines().last().unwrap();
    let mut steps = RE_INSTRUCTION_STEPS
        .find_iter(instructions_raw)
        .map(|step| step.as_str().parse::<i32>().unwrap());
    let mut turns = RE_INSTRUCTION_TURNS
        .find_iter(instructions_raw)
        .map(|direction| direction.as_str());

    loop {
        match (steps.next(), turns.next()) {
            (Some(step), Some(turn)) => {
                instructions.push(Instruction::Step(step));
                instructions.push(Instruction::Turn(turn))
            }
            (Some(step), None) => instructions.push(Instruction::Step(step)),
            (None, Some(turn)) => instructions.push(Instruction::Turn(turn)),
            (None, None) => break,
        }
    }

    // parse map
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let map_raw = contents.split("\n\n").collect::<Vec<&str>>()[0]
        .lines()
        .collect::<Vec<&str>>();
    for (row, tiles) in map_raw.iter().enumerate() {
        for (col, tile) in tiles.chars().enumerate() {
            if tile == '.' || tile == '#' {
                map.entry((i32::try_from(row).unwrap(), i32::try_from(col).unwrap()))
                    .or_insert(tile);
            }
        }
    }

    // get start position
    let start_r = map
        .iter()
        .sorted_by(|a, b| b.0 .0.cmp(&a.0 .0))
        .last()
        .unwrap()
        .0
         .0;
    let start_c = map
        .iter()
        .filter(|&(key, _)| key.0 == start_r)
        .sorted_by(|a, b| b.0 .1.cmp(&a.0 .1))
        .last()
        .unwrap()
        .0
         .1;

    let mut current_state: State = State::new((start_r, start_c), Facing::Right);

    for instruction in instructions {
        current_state = instruction.translate(&current_state, &map);
    }

    return (1000 * (current_state.pos.0 + 1))
        + (4 * (current_state.pos.1 + 1))
        + current_state.facing.constant();
}
