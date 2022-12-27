use std::collections::{HashSet, VecDeque};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
struct Rock<'a> {
    formation: Vec<(i32, i32)>,
    _rock_type: &'a RockType,
}

impl<'a> Rock<'a> {
    fn new(rock_type: &'a RockType, highest_point: i32) -> Rock<'a> {
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
                    _rock_type: rock_type,
                }
            }
            RockType::Horizontal => {
                let mut formation = Vec::new();
                for col in 0..4 {
                    formation.push((highest_point + span_row, span_col + col));
                }
                Rock {
                    formation,
                    _rock_type: rock_type,
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
                    _rock_type: rock_type,
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
                    _rock_type: rock_type,
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
                    _rock_type: rock_type,
                }
            }
        }
    }

    fn edge(&self, direction: (i32, i32)) -> Vec<&(i32, i32)> {
        return self
            .formation
            .iter()
            .filter(|&(row, column)| {
                !self
                    .formation
                    .contains(&(row + direction.0, column + direction.1))
            })
            .collect::<Vec<&(i32, i32)>>();
    }

    fn translate(
        &mut self,
        direction: (i32, i32),
        surface: &HashSet<(i32, i32)>,
        width: i32,
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
                return can_move;
            }
            _ => false, // move up.  Illegal move, and not expected.
        }
    }
}

#[derive(PartialEq, Debug)]
enum RockType {
    Vertical,
    Horizontal,
    Square,
    Cross,
    Hook,
}

pub fn part_a(contents: &str) -> i32 {
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let width = 7;
    let mut surface: HashSet<(i32, i32)> = HashSet::new();
    let mut highest_point = 0;
    let mut hot_gas = VecDeque::from(
        contents
            .trim()
            .chars()
            .map(|c| match c {
                '<' => &directions[3],
                _ => &directions[1],
            })
            .collect::<Vec<&(i32, i32)>>(),
    );
    let mut rock_cycle = VecDeque::from([
        RockType::Horizontal,
        RockType::Cross,
        RockType::Hook,
        RockType::Vertical,
        RockType::Square,
    ]);

    for _ in 0..2022 {
        let mut r = Rock::new(rock(&mut rock_cycle), highest_point);
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

    return highest_point;
}

fn jet<'a>(hot_gas: &mut VecDeque<&'a (i32, i32)>) -> &'a (i32, i32) {
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
    surface: &HashSet<(i32, i32)>,
    highest_point: i32,
    width: i32,
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
        println!("{:?}", row);
    }
    println!();
}
