use aoc2019::computer::*;
use aoc2019::*;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    N = 1,
    S = 2,
    W = 3,
    E = 4,
}
use Direction::*;

impl Into<i64> for Direction {
    fn into(self) -> i64 {
        match self {
            N => 1,
            S => 2,
            W => 3,
            E => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn move_to(self, d: Direction) -> Self {
        match d {
            N => Point(self.0, self.1 + 1),
            S => Point(self.0, self.1 - 1),
            W => Point(self.0 - 1, self.1),
            E => Point(self.0 + 1, self.1),
        }
    }
}

#[derive(Clone)]
struct SearchState {
    inst: Instant,
    loc: Point,
    depth: usize,
}

fn q1(prog: &Program) -> Result<SearchState> {
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(Point(0, 0));
    let mut queue: VecDeque<SearchState> = VecDeque::new();
    queue.push_back(SearchState {
        inst: prog.start(),
        loc: Point(0, 0),
        depth: 0,
    });

    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        for &input in &[N, S, W, E] {
            let next_loc = state.loc.move_to(input);
            if !visited.insert(next_loc) {
                continue;
            }
            let mut next_state = state.clone();
            next_state.loc = next_loc;
            next_state.depth += 1;
            next_state.inst.push_input(input.into());
            match next_state.inst.step()? {
                StepResult::Halt | StepResult::WaitInput => continue,
                StepResult::Output => match next_state.inst.pop_output().unwrap() {
                    0 => continue,
                    1 => queue.push_back(next_state),
                    2 => return Ok(next_state),
                    _ => unreachable!(),
                },
            }
        }
    }
    bail!("failed to solve q1");
}

fn q2(mut start_state: SearchState) -> Result<usize> {
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(start_state.loc);
    let mut queue: VecDeque<SearchState> = VecDeque::new();
    start_state.depth = 0;
    queue.push_back(start_state);
    let mut max_depth = 0;

    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        max_depth = std::cmp::max(max_depth, state.depth);
        for &input in &[N, S, W, E] {
            let next_loc = state.loc.move_to(input);
            if !visited.insert(next_loc) {
                continue;
            }
            let mut next_state = state.clone();
            next_state.loc = next_loc;
            next_state.depth += 1;
            next_state.inst.push_input(input.into());
            match next_state.inst.step()? {
                StepResult::Halt | StepResult::WaitInput => continue,
                StepResult::Output => match next_state.inst.pop_output().unwrap() {
                    0 => continue,
                    1 | 2 => queue.push_back(next_state),
                    _ => unreachable!(),
                },
            }
        }
    }

    Ok(max_depth)
}

fn main() -> Result<()> {
    let prog = Program::load_from_input("day15.txt")?;

    let state = q1(&prog)?;
    println!("ans1={:?}", state.depth);

    let ans2 = q2(state)?;
    println!("ans2={:?}", ans2);

    Ok(())
}
