use aoc2019::computer::*;
use aoc2019::*;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Copy, Clone)]
enum Color {
    Black = 0,
    White = 1,
}

impl TryFrom<Int> for Color {
    type Error = Error;

    fn try_from(val: Int) -> Result<Color> {
        match val {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => bail!("invalid color"),
        }
    }
}

impl Into<Int> for Color {
    fn into(self) -> Int {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point(i32, i32);

struct Robot {
    instant: Instant,
    direction: Direction,
    loc: Point,
    paints: HashMap<Point, Color>,
}

impl Robot {
    fn new(prog: &Program) -> Self {
        Self {
            instant: prog.start(),
            direction: Direction::N,
            loc: Point(0, 0),
            paints: HashMap::new(),
        }
    }

    fn get_color(&self, loc: Point) -> Color {
        *self.paints.get(&loc).unwrap_or(&Color::Black)
    }

    fn push_input(&mut self, val: Int) {
        self.instant.input.push_back(val);
    }

    fn pop_output(&mut self) -> Result<Int> {
        self.instant
            .output
            .pop_front()
            .ok_or_else(|| anyhow!("failed to get output"))
    }

    fn run(&mut self, color: Color) -> Result<()> {
        self.push_input(color.into());
        loop {
            if self.instant.step()? == StepResult::Halt {
                break;
            }
            let color = Color::try_from(self.pop_output()?)?;
            self.paints.insert(self.loc, color);
            if self.instant.step()? == StepResult::Halt {
                break;
            }
            let turn = self.pop_output()?;
            self.direction = match turn {
                0 => match self.direction {
                    Direction::N => Direction::W,
                    Direction::E => Direction::N,
                    Direction::S => Direction::E,
                    Direction::W => Direction::S,
                },
                1 => match self.direction {
                    Direction::N => Direction::E,
                    Direction::E => Direction::S,
                    Direction::S => Direction::W,
                    Direction::W => Direction::N,
                },
                _ => bail!("invalid turn"),
            };
            match self.direction {
                Direction::N => self.loc.1 -= 1,
                Direction::E => self.loc.0 += 1,
                Direction::S => self.loc.1 += 1,
                Direction::W => self.loc.0 -= 1,
            }
            self.push_input(self.get_color(self.loc).into());
        }

        Ok(())
    }

    fn paint_region(&self) -> (i32, i32, i32, i32) {
        use std::cmp::{max, min};
        let mut min_x: i32 = 0;
        let mut max_x: i32 = 0;
        let mut min_y: i32 = 0;
        let mut max_y: i32 = 0;
        for p in self.paints.keys() {
            min_x = min(p.0, min_x);
            max_x = max(p.0, max_x);
            min_y = min(p.1, min_y);
            max_y = max(p.1, max_y);
        }
        (min_x, max_x, min_y, max_y)
    }
}

fn main() -> Result<()> {
    let prog = Program::load_from_input("day11.txt")?;

    let mut robot = Robot::new(&prog);
    robot.run(Color::Black)?;
    println!("ans1={:?}", robot.paints.len());

    let mut robot = Robot::new(&prog);
    robot.run(Color::White)?;
    let region = robot.paint_region();
    for y in region.2..=region.3 {
        for x in region.0..=region.1 {
            match robot.get_color(Point(x, y)) {
                Color::Black => print!("\x1B[40m "),
                Color::White => print!("\x1B[107m "),
            }
        }
        println!("\x1B[m");
    }

    Ok(())
}
