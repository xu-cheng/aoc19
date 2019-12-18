use aoc2019::computer::*;
use aoc2019::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    N = 1,
    S = 2,
    W = 3,
    E = 4,
}
use Direction::*;

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            N => W,
            S => E,
            W => S,
            E => N,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            N => E,
            S => W,
            W => N,
            E => S,
        }
    }

    fn turn(self, d: char) -> Self {
        match d {
            'L' => self.turn_left(),
            'R' => self.turn_right(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point(i32, i32);

impl Point {
    fn move_to(self, d: Direction, step: i32) -> Self {
        match d {
            N => Point(self.0, self.1 - step),
            S => Point(self.0, self.1 + step),
            W => Point(self.0 - step, self.1),
            E => Point(self.0 + step, self.1),
        }
    }
}

struct View {
    data: Vec<u8>,
    width: i32,
    height: i32,
}

impl View {
    fn new(input: &[Int]) -> Self {
        let width = input.iter().enumerate().find(|(_, &c)| c == 10).unwrap().0 as i32;
        let data: Vec<u8> = input
            .iter()
            .filter_map(|&c| if c == 10 { None } else { Some(c as u8) })
            .collect();
        let height = data.len() as i32 / width;

        Self {
            data,
            width,
            height,
        }
    }

    fn try_get(&self, p: Point) -> Option<u8> {
        if p.0 >= 0 && p.0 < self.width && p.1 >= 0 && p.1 < self.height {
            Some(self.data[(p.1 * self.width + p.0) as usize])
        } else {
            None
        }
    }

    fn get(&self, p: Point) -> u8 {
        self.try_get(p).unwrap()
    }

    fn intersecs(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        for x in 1..self.width - 1 {
            for y in 1..self.height - 1 {
                if self.get(Point(x, y)) == 35
                    && self.get(Point(x + 1, y)) == 35
                    && self.get(Point(x - 1, y)) == 35
                    && self.get(Point(x, y + 1)) == 35
                    && self.get(Point(x, y - 1)) == 35
                {
                    result.insert(Point(x, y));
                }
            }
        }
        result
    }

    fn robot(&self) -> (Point, Direction) {
        let idx = self
            .data
            .iter()
            .position(|&c| {
                let c = c as char;
                c == '^' || c == '<' || c == '>' || c == 'v'
            })
            .unwrap() as i32;
        let p = Point(idx % self.width, idx / self.width);
        let d = match self.data[idx as usize] as char {
            '^' => N,
            'v' => S,
            '>' => E,
            '<' => W,
            _ => unreachable!(),
        };
        (p, d)
    }

    fn robot_path(&self) -> Vec<Move> {
        let mut path: Vec<Move> = Vec::new();
        let (mut cur_p, mut cur_d) = self.robot();

        loop {
            let mut new_d: Option<Direction> = None;
            let mut turn: char = 'L';
            for &t in &['L', 'R'] {
                let d = cur_d.turn(t);
                let p = cur_p.move_to(d, 1);
                if self.try_get(p) == Some(35) {
                    new_d = Some(d);
                    turn = t;
                    break;
                }
            }

            match new_d {
                Some(d) => cur_d = d,
                None => break,
            }

            let mut step = 0;
            while self.try_get(cur_p.move_to(cur_d, step + 1)) == Some(35) {
                step += 1;
            }
            cur_p = cur_p.move_to(cur_d, step);
            path.push(Move(turn, step));
        }

        path
    }
}

// Ref: https://en.wikipedia.org/wiki/Byte_pair_encoding

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Move(char, i32);

impl Move {
    fn to_input(self) -> Vec<Int> {
        let mut res = Vec::new();
        res.push(self.0 as Int);
        res.push(44);
        for c in self.1.to_string().chars() {
            res.push(c as Int);
        }
        res.push(44);
        res
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Path {
    Single(Move),
    Multi(Vec<Move>),
}

impl Path {
    fn len(&self) -> usize {
        match self {
            Path::Single(_) => 4,
            Path::Multi(x) => x.len() * 4,
        }
    }

    fn to_input(&self) -> Vec<Int> {
        let mut res = match self {
            Path::Single(x) => x.to_input(),
            Path::Multi(x) => {
                let mut res: Vec<Int> = Vec::new();
                for m in x {
                    res.append(&mut m.to_input());
                }
                res
            }
        };
        res.pop();
        res.push(10);
        res
    }
}

fn compress_path(input: &[Move]) -> Result<Vec<Path>> {
    let mut cur_path: Vec<Path> = input.iter().map(|&m| Path::Single(m)).collect();

    while cur_path.iter().collect::<HashSet<_>>().len() > 3 {
        let mut counter: HashMap<(bool, Path, Path), usize> = HashMap::new();
        for i in 0..cur_path.len() - 1 {
            let pair = (i % 2 == 0, cur_path[i].clone(), cur_path[i + 1].clone());
            if pair.1.len() + pair.2.len() <= 20 {
                *counter.entry(pair).or_insert(0) += 1;
            }
        }
        let top = counter
            .iter()
            .max_by(|((_, p11, p12), cnt1), ((_, p21, p22), cnt2)| {
                if *cnt1 == *cnt2 {
                    let len1 = p11.len() + p12.len();
                    let len2 = p21.len() + p22.len();
                    len1.cmp(&len2).reverse()
                } else {
                    cnt1.cmp(cnt2)
                }
            })
            .context("failed to find solution")?
            .0;
        let mut top_path: Vec<Move> = Vec::new();
        match &top.1 {
            Path::Single(x) => top_path.push(*x),
            Path::Multi(x) => top_path.append(&mut x.clone()),
        }
        match &top.2 {
            Path::Single(x) => top_path.push(*x),
            Path::Multi(x) => top_path.append(&mut x.clone()),
        }
        let mut new_path: Vec<Path> = Vec::with_capacity(cur_path.len());
        let mut i = 0;
        while i < cur_path.len() {
            if i < cur_path.len() - 1
                && (i % 2 == 0) == top.0
                && cur_path[i] == top.1
                && cur_path[i + 1] == top.2
            {
                new_path.push(Path::Multi(top_path.clone()));
                i += 2;
            } else {
                new_path.push(cur_path[i].clone());
                i += 1;
            }
        }
        cur_path = new_path;
    }

    Ok(cur_path)
}

fn main() -> Result<()> {
    let mut prog = Program::load_from_input("day17.txt")?;
    let view = View::new(&prog.start().execute()?);

    let ans1: i32 = view.intersecs().iter().map(|p| p.0 * p.1).sum();
    println!("ans1={:?}", ans1);

    let path = view.robot_path();
    let cpath = (0..100)
        .filter_map(|_| compress_path(&path).ok())
        .next()
        .context("too many attempts")?;
    let dict = cpath.iter().collect::<HashSet<_>>();
    let mut dict_iter = dict.iter();
    let fn_a = dict_iter.next().unwrap();
    let fn_b = dict_iter.next().unwrap();
    let fn_c = dict_iter.next().unwrap();
    let mut main_input: Vec<Int> = Vec::new();
    for p in &cpath {
        if !main_input.is_empty() {
            main_input.push(44);
        }
        if &p == fn_a {
            main_input.push('A' as Int);
        } else if &p == fn_b {
            main_input.push('B' as Int);
        } else if &p == fn_c {
            main_input.push('C' as Int);
        } else {
            bail!("invalid path");
        }
    }
    main_input.push(10);
    let fn_a_input = fn_a.to_input();
    let fn_b_input = fn_b.to_input();
    let fn_c_input = fn_c.to_input();

    prog.0[0] = 2;
    let mut robot = prog.start();
    robot.push_inputs(&main_input);
    robot.push_inputs(&fn_a_input);
    robot.push_inputs(&fn_b_input);
    robot.push_inputs(&fn_c_input);
    robot.push_input('n' as Int);
    robot.push_input(10);
    let output = robot.execute()?;
    println!("ans2={:?}", output.last().unwrap());

    Ok(())
}
