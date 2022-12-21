use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::ops::RangeInclusive;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }

    fn manhattan(&self, other: &Self) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }
}

pub fn part_a(contents: &str) -> i32 {
    lazy_static! {
        static ref RE_SENSOR: Regex = Regex::new(r"Sensor.*\bx=(\d{1,}), y=(\d{1,})").unwrap();
        static ref RE_BEACON: Regex = Regex::new(r"beacon.*\bx=(\d{1,}), y=(\d{1,})").unwrap();
        static ref RE_XY: Regex = Regex::new(r"-?\d{1,}").unwrap();
    }

    let mut pairs: Vec<(Point, Point)> = Vec::new();

    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    let mut min_x: i32 = i32::MAX;
    let mut min_y: i32 = i32::MAX;

    for line in contents.lines() {
        let mut coords = RE_XY.find_iter(line);

        // sensor
        let mut x = coords.next().unwrap().as_str().parse::<i32>().unwrap();
        let mut y = coords.next().unwrap().as_str().parse::<i32>().unwrap();
        max_x = std::cmp::max(max_x, x);
        max_y = std::cmp::max(max_y, y);
        min_x = std::cmp::min(min_x, x);
        min_y = std::cmp::min(min_y, y);
        let sensor = Point::new(x, y);

        // beacon
        x = coords.next().unwrap().as_str().parse::<i32>().unwrap();
        y = coords.next().unwrap().as_str().parse::<i32>().unwrap();
        max_x = std::cmp::max(max_x, x);
        max_y = std::cmp::max(max_y, y);
        min_x = std::cmp::min(min_x, x);
        min_y = std::cmp::min(min_y, y);
        let beacon = Point::new(x, y);

        pairs.push((sensor, beacon));
    }

    let y = 2_000_000;
    let mut max_cx: i32 = 0;
    let mut min_cx: i32 = i32::MAX;
    let mut row_beacons: HashSet<&Point> = HashSet::new();
    let mut row_coverage: Vec<RangeInclusive<i32>> = Vec::new();
    for idx in 0..pairs.len() {
        let mdb = pairs[idx].0.manhattan(&pairs[idx].1);
        let mdl = pairs[idx].0.manhattan(&Point::new(pairs[idx].0.x, y));
        if mdl <= mdb {
            let mdr_offset = (mdb - (pairs[idx].0.y - y).abs()).abs();
            let cx_start = pairs[idx].0.x - mdr_offset;
            let cx_end = pairs[idx].0.x + mdr_offset;
            min_cx = std::cmp::min(min_cx, cx_start);
            max_cx = std::cmp::max(max_cx, cx_end);
            row_coverage.push(cx_start..=cx_end);
            if pairs[idx].1.y == y {
                row_beacons.insert(&pairs[idx].1);
            }
        }
    }

    let mut covered_positions = 0;
    for position in min_cx..=max_cx {
        for rc in row_coverage.iter() {
            if rc.contains(&position) {
                covered_positions += 1;
                break;
            }
        }
    }

    return covered_positions - i32::try_from(row_beacons.len()).unwrap();
}
