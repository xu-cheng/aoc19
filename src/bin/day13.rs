use aoc2019::computer::*;
use aoc2019::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point(i64, i64);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Wall,
    Block,
    HPaddle,
    Ball,
}

impl TryFrom<i64> for Tile {
    type Error = Error;

    fn try_from(val: i64) -> Result<Self> {
        use Tile::*;
        Ok(match val {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => HPaddle,
            4 => Ball,
            _ => bail!("invalid tile"),
        })
    }
}

struct Game {
    instant: Instant,
    tiles: HashMap<Point, Tile>,
    score: i64,
}

impl Game {
    fn new(prog: &Program, free_to_play: bool) -> Self {
        let mut instant = prog.start();
        if free_to_play {
            instant.mem[0] = 2;
        }
        Self {
            instant,
            tiles: HashMap::new(),
            score: 0,
        }
    }

    fn step(&mut self) -> Result<bool> {
        self.tiles.clear();
        let is_halt = loop {
            match self.instant.step()? {
                StepResult::Halt => break true,
                StepResult::WaitInput => break false,
                StepResult::Output => continue,
            }
        };
        for mut chunk in &self.instant.output.iter().chunks(3) {
            let x = *chunk.next().context("failed to read output")?;
            let y = *chunk.next().context("failed to read output")?;
            let t = *chunk.next().context("failed to read output")?;
            match Tile::try_from(t) {
                Ok(tile) => {
                    self.tiles.insert(Point(x, y), tile);
                }
                Err(_) => {
                    self.score = t;
                }
            }
        }
        self.instant.output.clear();
        Ok(is_halt)
    }

    fn send_input(&mut self, val: i64) {
        self.instant.input.push_back(val);
    }

    fn blocks(&self) -> impl Iterator<Item = (&Point, &Tile)> {
        self.tiles.iter().filter(|(_, &t)| t == Tile::Block)
    }

    fn ball(&self) -> Point {
        self.tiles
            .iter()
            .find(|(_, &t)| t == Tile::Ball)
            .map(|(&p, _)| p)
            .unwrap_or_else(|| Point(0, 0))
    }

    fn paddle(&self) -> Point {
        self.tiles
            .iter()
            .find(|(_, &t)| t == Tile::HPaddle)
            .map(|(&p, _)| p)
            .unwrap_or_else(|| Point(0, 0))
    }
}

fn main() -> Result<()> {
    let prog = Program::load_from_input("day13.txt")?;

    let mut game1 = Game::new(&prog, false);
    while !game1.step()? {}
    let ans1 = game1.blocks().count();
    println!("ans1={:?}", ans1);

    let mut game2 = Game::new(&prog, true);
    while !game2.step()? {
        let paddle = game2.paddle();
        let ball = game2.ball();
        let input = match ball.0.cmp(&paddle.0) {
            Ordering::Equal => 0,
            Ordering::Greater => 1,
            Ordering::Less => -1,
        };
        game2.send_input(input);
    }
    println!("ans2={:?}", game2.score);

    Ok(())
}
