use std::cmp::Ordering;

#[derive(Debug)]
enum Packet {
    Literal(usize),
    Stream(Vec<Packet>),
}

impl Packet {
    fn deserialize(stream_packed: &str) -> Packet {
        let stream = &mut stream_packed.chars();
        let mut parser: Vec<Vec<Packet>> = Vec::new();
        let result: Vec<Packet> = Vec::new();
        let mut literal: (Option<usize>, usize) = (None, 0);
        parser.push(result);
        stream.next();

        while let Some(c) = stream.next() {
            match c {
                '[' => {
                    let p: Vec<Packet> = Vec::new();
                    parser.push(p);
                }
                ']' => {
                    if literal.0 != None {
                        let index = parser.len() - 1;
                        parser[index].push(Self::Literal(literal.0.unwrap()));
                        literal = (None, 0);
                    }

                    let s = Self::Stream(parser.pop().unwrap());
                    if parser.len() == 0 {
                        return s;
                    }

                    let index = parser.len() - 1;
                    parser[index].push(s);
                }
                ',' => {
                    if literal.0 != None {
                        let index = parser.len() - 1;
                        parser[index].push(Self::Literal(literal.0.unwrap()));
                        literal = (None, 0);
                    }
                }
                _ => {
                    if literal.0 == None {
                        literal.0 = Some(
                            (c as usize - '0' as usize)
                                * usize::try_from(10)
                                    .unwrap()
                                    .pow(u32::try_from(literal.1).unwrap()),
                        );
                    } else {
                        literal.0 = Some(
                            (literal.0.unwrap()
                                * usize::try_from(10)
                                    .unwrap()
                                    .pow(u32::try_from(literal.1).unwrap()))
                                + (c as usize - '0' as usize),
                        );
                    }
                    literal.1 += 1;
                }
            }
        }

        return Self::Stream(vec![]);
    }

    fn compare(&self, second: &Self) -> Ordering {
        let order = match (self, second) {
            (Packet::Literal(left), Packet::Literal(right)) => {
                if left < right {
                    Ordering::Less
                } else if left > right {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (Packet::Stream(left), Packet::Stream(right)) => {
                let shortest = std::cmp::min(left.len(), right.len());
                let mut order = Ordering::Equal;
                for index in 0..shortest {
                    order = left[index].compare(&right[index]);
                    if order == Ordering::Less || order == Ordering::Greater {
                        break;
                    }
                }
                if order == Ordering::Equal {
                    if left.len() < right.len() {
                        Ordering::Less
                    } else if left.len() > right.len() {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                } else {
                    order
                }
            }
            (Packet::Literal(left), Packet::Stream(_)) => {
                Packet::Stream(vec![Packet::Literal(*left)]).compare(second)
            }

            (Packet::Stream(_), Packet::Literal(right)) => {
                self.compare(&Packet::Stream(vec![Packet::Literal(*right)]))
            }
        };

        return order;
    }
}

pub fn part_b(contents: &str) -> i32 {
    let mut distress_call = contents
        .lines()
        .filter(|x| x.trim().len() > 0)
        .map(|y| Packet::deserialize(y))
        .collect::<Vec<Packet>>();

    distress_call.push(Packet::deserialize("[[2]]"));
    distress_call.push(Packet::deserialize("[[6]]"));

    distress_call.sort_by(|a, b| a.compare(b));

    let mut decoder = 1;
    for (index, packet) in distress_call.iter().enumerate() {
        decoder *= if packet.compare(&Packet::deserialize("[[2]]")) == Ordering::Equal {
            index + 1
        } else if packet.compare(&Packet::deserialize("[[6]]")) == Ordering::Equal {
            index + 1
        } else {
            1
        }
    }

    return decoder as i32;
}
