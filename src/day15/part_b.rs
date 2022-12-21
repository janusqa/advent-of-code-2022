use lazy_static::lazy_static;
use regex::Regex;
use std::{convert::TryFrom, ops::RangeInclusive};

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        return Point { x, y };
    }

    fn manhattan(&self, other: &Self) -> i64 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }
}

pub fn part_b(contents: &str) -> i64 {
    lazy_static! {
        static ref RE_SENSOR: Regex = Regex::new(r"Sensor.*\bx=(\d{1,}), y=(\d{1,})").unwrap();
        static ref RE_BEACON: Regex = Regex::new(r"beacon.*\bx=(\d{1,}), y=(\d{1,})").unwrap();
        static ref RE_XY: Regex = Regex::new(r"-?\d{1,}").unwrap();
    }

    let mut pairs: Vec<(Point, Point)> = Vec::new();

    let mut max_x: i64 = 0;
    let mut max_y: i64 = 0;
    let mut min_x: i64 = i64::MAX;
    let mut min_y: i64 = i64::MAX;

    for line in contents.lines() {
        let mut coords = RE_XY.find_iter(line);

        // sensor
        let mut x = coords.next().unwrap().as_str().parse::<i64>().unwrap();
        let mut y = coords.next().unwrap().as_str().parse::<i64>().unwrap();
        max_x = std::cmp::max(max_x, x);
        max_y = std::cmp::max(max_y, y);
        min_x = std::cmp::min(min_x, x);
        min_y = std::cmp::min(min_y, y);
        let sensor = Point::new(x, y);
        // beacon
        x = coords.next().unwrap().as_str().parse::<i64>().unwrap();
        y = coords.next().unwrap().as_str().parse::<i64>().unwrap();
        max_x = std::cmp::max(max_x, x);
        max_y = std::cmp::max(max_y, y);
        min_x = std::cmp::min(min_x, x);
        min_y = std::cmp::min(min_y, y);
        let beacon = Point::new(x, y);

        pairs.push((sensor, beacon));
    }

    let mut tuning_frequency = 0;
    let coefficient = 4_000_000;
    let max_size = 4_000_001;

    // Calculate coverage of scanners
    let y_range = 0..i64::try_from(max_size).unwrap();
    for y in y_range {
        let mut row_coverage: Vec<RangeInclusive<i64>> = Vec::new();
        for idx in 0..pairs.len() {
            let mdb = pairs[idx].0.manhattan(&pairs[idx].1);
            let mdl = pairs[idx].0.manhattan(&Point::new(pairs[idx].0.x, y));
            if mdl <= mdb {
                let mdr_offset = (mdb - (pairs[idx].0.y - y).abs()).abs();
                let cx_start = std::cmp::max(0, pairs[idx].0.x - mdr_offset);
                let cx_end = std::cmp::min(
                    i64::try_from(max_size - 1).unwrap(),
                    pairs[idx].0.x + mdr_offset,
                );
                row_coverage.push(cx_start..=cx_end);
            }
            row_coverage.sort_by(|a, b| a.start().cmp(&b.start()))
        }

        // find distress becon by search for a gap in the coverage ranges
        // be mindful of gaps of 0 length (i.e. ranges that sit side by side) and ignore them
        // only consider gaps that have a space of 1 grid space between them.
        let mut merged = row_coverage[0].clone();
        for idx in 1..row_coverage.len() {
            // check to see if ranges sit directly adjacent to each other
            // if they do consider them to be overlapped
            if (row_coverage[idx].start() - merged.end()).abs() == 1 {
                merged = RangeInclusive::new(*merged.start(), *merged.end() + 1);
            }

            let m = merge_coverage(&merged, &row_coverage[idx]);
            if m == None {
                tuning_frequency = (*merged.end() + 1) * coefficient + y;
                break;
            }
            merged = m.unwrap();
        }
    }

    return tuning_frequency;
}

fn merge_coverage(
    r1: &RangeInclusive<i64>,
    r2: &RangeInclusive<i64>,
) -> Option<RangeInclusive<i64>> {
    // two ranges overlap/merge if (StartA <= EndB) && (StartB <= EndA)
    if (r1.start() <= r2.end()) && (r2.start() <= r1.end()) {
        return Some(std::cmp::min(*r1.start(), *r2.start())..=std::cmp::max(*r1.end(), *r2.end()));
    }

    return None;
}
