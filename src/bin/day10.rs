extern crate num_integer;

use aoc2019::*;
use num_integer::Integer;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point(i32, i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Angle(i32, i32);

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum Direction {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
}

impl Angle {
    fn from_points(src: Point, dst: Point) -> Self {
        let delta_x = dst.0 - src.0;
        let delta_y = dst.1 - src.1;
        if delta_x == 0 {
            Self(0, delta_y.signum())
        } else if delta_y == 0 {
            Self(delta_x.signum(), 0)
        } else {
            let gcd = delta_x.abs().gcd(&delta_y.abs());
            Self(delta_x / gcd, delta_y / gcd)
        }
    }

    fn to_direction(self) -> Direction {
        match (self.0.signum(), self.1.signum()) {
            (0, -1) => Direction::N,
            (1, -1) => Direction::NE,
            (1, 0) => Direction::E,
            (1, 1) => Direction::SE,
            (0, 1) => Direction::S,
            (-1, 1) => Direction::SW,
            (-1, 0) => Direction::W,
            (-1, -1) => Direction::NW,
            _ => unreachable!(),
        }
    }
}

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else {
            let dir1 = self.to_direction();
            let dir2 = other.to_direction();
            if dir1 == dir2 {
                let tan1 = self.0 as f64 / self.1 as f64;
                let tan2 = other.0 as f64 / other.1 as f64;
                tan2.partial_cmp(&tan1).unwrap()
            } else {
                dir1.cmp(&dir2)
            }
        }
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Target {
    p: Point,
    d: i32,
}

impl Ord for Target {
    fn cmp(&self, other: &Self) -> Ordering {
        other.d.cmp(&self.d)
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<()> {
    let reader = open_input("day10.txt")?;
    let mut asteroids: Vec<Point> = Vec::new();
    for (y, line) in reader.lines().enumerate() {
        for (x, p) in line?.trim().chars().enumerate() {
            if p == '#' {
                asteroids.push(Point(x as i32, y as i32));
            }
        }
    }

    let (station, ans1) = asteroids
        .iter()
        .map(|&src| {
            let num = asteroids
                .iter()
                .filter(|&dst| src != *dst)
                .map(|&dst| Angle::from_points(src, dst))
                .collect::<HashSet<Angle>>()
                .len();
            (src, num)
        })
        .max_by_key(|&(_, num)| num)
        .unwrap();
    println!("ans1={:?}", ans1);

    let mut targets: BTreeMap<Angle, BinaryHeap<Target>> = BTreeMap::new();
    for p in &asteroids {
        if *p == station {
            continue;
        }
        let angle = Angle::from_points(station, *p);
        let delta_x = station.0 - p.0;
        let delta_y = station.1 - p.1;
        let dist = delta_x * delta_x + delta_y * delta_y;
        targets
            .entry(angle)
            .or_insert_with(BinaryHeap::new)
            .push(Target { p: *p, d: dist });
    }

    let mut cnt = 0;
    let ans2 = 'outer: loop {
        let mut keys_to_be_deleted: Vec<Angle> = Vec::new();
        for (&k, ts) in targets.iter_mut() {
            if let Some(t) = ts.pop() {
                cnt += 1;
                if cnt == 200 {
                    break 'outer t.p.0 * 100 + t.p.1;
                }
            }
            if ts.is_empty() {
                keys_to_be_deleted.push(k);
            }
        }
        for k in &keys_to_be_deleted {
            targets.remove(k);
        }
    };
    println!("ans2={:?}", ans2);

    Ok(())
}
