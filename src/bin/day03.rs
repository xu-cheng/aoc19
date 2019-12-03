use aoc2019::*;
use std::collections::{HashMap, HashSet};

fn parse_wire(s: &str) -> HashMap<(i32, i32), i32> {
    let mut out: HashMap<(i32, i32), i32> = HashMap::new();
    let mut cur = (0, 0);
    let mut step = 0;
    for path in s.trim().split(',') {
        let direction = &path[0..1];
        let distant = path[1..].parse::<i32>().unwrap();
        let next_point: Box<dyn Fn(&mut (i32, i32))> = match direction {
            "U" => Box::new(|pt: &mut (i32, i32)| pt.1 += 1),
            "D" => Box::new(|pt: &mut (i32, i32)| pt.1 -= 1),
            "L" => Box::new(|pt: &mut (i32, i32)| pt.0 -= 1),
            "R" => Box::new(|pt: &mut (i32, i32)| pt.0 += 1),
            _ => unreachable!(),
        };
        for _ in 0..distant {
            next_point(&mut cur);
            step += 1;
            out.insert(cur.clone(), step);
        }
    }
    out.insert(cur, step);
    out
}

fn main() -> Result<()> {
    let mut reader = open_input("day03.txt")?;
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let wire1 = parse_wire(&buf);
    buf.clear();
    reader.read_line(&mut buf)?;
    let wire2 = parse_wire(&buf);

    let wire1_pts: HashSet<(i32, i32)> = wire1.keys().cloned().collect();
    let wire2_pts: HashSet<(i32, i32)> = wire2.keys().cloned().collect();
    let intersections: HashSet<(i32, i32)> = wire1_pts.intersection(&wire2_pts).cloned().collect();

    let ans1 = intersections.iter().map(|&pt| pt.0 + pt.1).min().unwrap();
    println!("ans1={}", ans1);

    let ans2 = intersections
        .iter()
        .map(|pt| wire1.get(pt).unwrap() + wire2.get(pt).unwrap())
        .min()
        .unwrap();
    println!("ans2={}", ans2);

    Ok(())
}
