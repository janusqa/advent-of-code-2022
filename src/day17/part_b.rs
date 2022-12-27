use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
struct Rock<'a> {
    formation: Vec<(i64, i64)>,
    rock_type: &'a RockType,
}

impl<'a> Rock<'a> {
    fn new(rock_type: &'a RockType, highest_point: i64) -> Rock<'a> {
        let span_row = 4;
        let span_col = 2;
        match rock_type {
            RockType::Vertical => {
                let mut formation = Vec::new();
                for row in 0..4 {
                    formation.push((highest_point + span_row + row, span_col));
                }
                Rock {
                    formation,
                    rock_type,
                }
            }
            RockType::Horizontal => {
                let mut formation = Vec::new();
                for col in 0..4 {
                    formation.push((highest_point + span_row, span_col + col));
                }
                Rock {
                    formation,
                    rock_type,
                }
            }
            RockType::Square => {
                let mut formation = Vec::new();
                for row in 0..2 {
                    for col in 0..2 {
                        formation.push((highest_point + span_row + row, span_col + col));
                    }
                }
                Rock {
                    formation,
                    rock_type,
                }
            }
            RockType::Cross => {
                let mut formation = Vec::new();
                for row in 0..3 {
                    for col in 0..3 {
                        if row == 1 {
                            formation.push((highest_point + span_row + row, span_col + col));
                        } else {
                            if col == 1 {
                                formation.push((highest_point + span_row + row, span_col + col));
                            }
                        }
                    }
                }
                Rock {
                    formation,
                    rock_type,
                }
            }
            RockType::Hook => {
                let mut formation = Vec::new();
                for row in 0..3 {
                    for col in 0..3 {
                        if row < 1 {
                            formation.push((highest_point + span_row + row, span_col + col));
                        } else {
                            if col > 1 {
                                formation.push((highest_point + span_row + row, span_col + col));
                            }
                        }
                    }
                }
                Rock {
                    formation,
                    rock_type,
                }
            }
        }
    }

    fn edge(&self, direction: (i64, i64)) -> Vec<&(i64, i64)> {
        return self
            .formation
            .iter()
            .filter(|&(row, column)| {
                !self
                    .formation
                    .contains(&(row + direction.0, column + direction.1))
            })
            .collect::<Vec<&(i64, i64)>>();
    }

    fn translate(
        &mut self,
        direction: (i64, i64),
        surface: &HashSet<(i64, i64)>,
        width: i64,
    ) -> bool {
        match direction {
            (0, 1) => {
                // move right
                let can_move = self.edge(direction).iter().all(|&(r, c)| {
                    !surface.contains(&(*r, *c + direction.1)) && c + direction.1 < width
                });
                if can_move {
                    for edge in self.formation.iter_mut() {
                        edge.1 += direction.1;
                    }
                }
                return can_move;
            }
            (-1, 0) => {
                // move down
                let can_move = self.edge(direction).iter().all(|&(r, c)| {
                    !surface.contains(&(*r + direction.0, *c)) && r + direction.0 > 0
                });
                if can_move {
                    for edge in self.formation.iter_mut() {
                        edge.0 += direction.0;
                    }
                }
                return can_move;
            }
            (0, -1) => {
                // move left
                let can_move = self.edge(direction).iter().all(|&(r, c)| {
                    !surface.contains(&(*r, *c + direction.1)) && c + direction.1 >= 0
                });
                if can_move {
                    for edge in self.formation.iter_mut() {
                        edge.1 += direction.1;
                    }
                }
                //println!("can move:{}", can_move);
                return can_move;
            }
            _ => false, // move up.  Illegal move, and not expected.
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum RockType {
    Vertical,
    Horizontal,
    Square,
    Cross,
    Hook,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct StateKey {
    rock_type: RockType,
    hot_gas: Vec<(i64, i64)>,
}

impl StateKey {
    fn new(rock_type: &RockType, hot_gas: &VecDeque<&(i64, i64)>) -> Self {
        return StateKey {
            rock_type: rock_type.clone(),
            hot_gas: hot_gas
                .clone()
                .iter()
                .map(|&(r, c)| (*r, *c))
                .collect::<Vec<(i64, i64)>>(),
        };
    }
}

pub fn part_b(contents: &str) -> i64 {
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let width = 7;
    let mut surface: HashSet<(i64, i64)> = HashSet::new();
    let mut highest_point = 0;
    let mut memo: HashMap<StateKey, (i64, i64)> = HashMap::new();
    let mut hot_gas = VecDeque::from(
        contents
            .trim()
            .chars()
            .map(|c| match c {
                '<' => &directions[3],
                _ => &directions[1],
            })
            .collect::<Vec<&(i64, i64)>>(),
    );
    let mut rock_cycle = VecDeque::from([
        RockType::Horizontal,
        RockType::Cross,
        RockType::Hook,
        RockType::Vertical,
        RockType::Square,
    ]);

    // We cannot loop through 1 trillion in a reasonable time.  It's possible, but it could take at least 1 day.
    // Therefore we need to look for a cycle. Not just a cycle but one where each cycle is adjacent to each other
    // to the end of the loop AND the size of the loop 1_000_000_000_000, in this example, MINUS the times a rock is dropped before cycle is detected
    // must be divisible by the size of the cycle, so that we can perform a clean calculation, right to the end of 1 trillion iterations exactly.
    // The equation is
    // (((loop_size − start_of_cycle_1) ÷ (start_of_cycle_2 − start_of_cycle_1)) × (highest_point_of_cycle_2 − highest_point_of_cycle_1)) + higest_point_of_cycle_1
    // Concrete example for part 2 sample data (((1000000000000 - 15) + (50 - 15)) * (78 - 25)) + 25
    // Traditional memoization is used here for cycle detection
    let num_drops = 1_000_000_000_000i64;
    for i in 0..num_drops {
        let mut r = Rock::new(rock(&mut rock_cycle), highest_point);
        let key = StateKey::new(&r.rock_type, &hot_gas);

        if memo.contains_key(&key)
            && (num_drops - memo.get(&key).unwrap().0) % (i - memo.get(&key).unwrap().0) == 0
        {
            return (((num_drops - memo.get(&key).unwrap().0) / (i - memo.get(&key).unwrap().0))
                * (highest_point - memo.get(&key).unwrap().1))
                + memo.get(&key).unwrap().1;
        }
        memo.entry(key).or_insert((i, highest_point));

        loop {
            r.translate(*jet(&mut hot_gas), &surface, width);
            if !r.translate(directions[2], &surface, width) {
                let s = r;
                surface.extend(s.edge(directions[0]));
                surface.extend(s.edge(directions[1]));
                surface.extend(s.edge(directions[3]));
                highest_point =
                    highest_point.max(s.edge(directions[0]).iter().map(|(r, _)| *r).max().unwrap());

                break;
            }
        }
    }

    // _display(&surface, highest_point, width, None, "");

    return highest_point;
}

fn jet<'a>(hot_gas: &mut VecDeque<&'a (i64, i64)>) -> &'a (i64, i64) {
    let g = hot_gas.pop_front().unwrap();
    hot_gas.push_back(g);
    return hot_gas.back().unwrap();
}

fn rock<'a>(rock_cycle: &'a mut VecDeque<RockType>) -> &'a RockType {
    let rt = rock_cycle.pop_front().unwrap();
    rock_cycle.push_back(rt);
    return rock_cycle.back().unwrap();
}

fn _display(
    surface: &HashSet<(i64, i64)>,
    highest_point: i64,
    width: i64,
    rock: Option<Rock>,
    description: &str,
) {
    let width = usize::try_from(width).unwrap();
    let height = usize::try_from(highest_point).unwrap() + 1 + 4 + 4;
    let mut screen: Vec<Vec<char>> = vec![vec![' '; width]; height];

    for point in surface.iter() {
        screen[usize::try_from(point.0).unwrap()][usize::try_from(point.1).unwrap()] = '#';
    }

    match rock {
        Some(rock) => {
            for point in rock.formation.iter() {
                screen[usize::try_from(point.0).unwrap()][usize::try_from(point.1).unwrap()] = '@';
            }
        }
        None => (),
    }

    println!("{}", description);
    if description == "Rock settles" {
        println!("surface: {:?}", surface);
    }
    for row in screen.iter().rev() {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}
