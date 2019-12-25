use aoc2019::computer::*;
use aoc2019::*;
use std::collections::{HashSet, VecDeque};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    N = 1,
    S = 2,
    W = 3,
    E = 4,
}
use Direction::*;

impl Direction {
    fn to_move_cmd(self) -> String {
        match self {
            N => "north\n",
            S => "south\n",
            E => "east\n",
            W => "west\n",
        }
        .to_owned()
    }
}

impl TryFrom<&str> for Direction {
    type Error = Error;

    fn try_from(input: &str) -> Result<Direction> {
        let d = if input == "north" {
            N
        } else if input == "south" {
            S
        } else if input == "east" {
            E
        } else if input == "west" {
            W
        } else {
            bail!("invalid direction: {}", input);
        };
        Ok(d)
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
struct Location {
    name: String,
    desc: String,
    doors: Vec<Direction>,
    items: Vec<String>,
}

impl TryFrom<&str> for Location {
    type Error = Error;

    fn try_from(input: &str) -> Result<Location> {
        let mut name = String::new();
        let mut desc = String::new();
        let mut doors: Vec<Direction> = Vec::new();
        let mut items: Vec<String> = Vec::new();
        let mut lines = input.lines();
        while let Some(line) = lines.next() {
            if line.is_empty() || line == "Command?" {
                continue;
            } else if line.starts_with("==") {
                name = line.replace('=', "").trim().to_owned();
                desc = lines.next().unwrap().to_owned();
            } else if line == "Doors here lead:" {
                lines
                    .take_while_ref(|l| l.starts_with('-'))
                    .map(|l| {
                        doors.push(Direction::try_from(l.replace("- ", "").as_str())?);
                        Ok(())
                    })
                    .collect::<Result<_>>()?;
            } else if line == "Items here:" {
                lines.take_while_ref(|l| l.starts_with('-')).for_each(|l| {
                    items.push(l.replace("- ", ""));
                })
            }
        }
        Ok(Location {
            name,
            desc,
            doors,
            items,
        })
    }
}

#[derive(Clone)]
struct Droid {
    instnt: Instant,
    loc: Location,
    items: Vec<String>,
}

impl Droid {
    fn new(prog: &Program) -> Result<Self> {
        let mut droid = Self {
            instnt: prog.start(),
            loc: Default::default(),
            items: Vec::new(),
        };
        let loc = Location::try_from(droid.reponse()?.as_str())?;
        droid.loc = loc;
        Ok(droid)
    }

    fn reponse(&mut self) -> Result<String> {
        loop {
            match self.instnt.step()? {
                StepResult::Halt => {
                    let out: String = self
                        .instnt
                        .output_iter()
                        .map(|c| *c as u8 as char)
                        .collect();
                    println!("{}", out);
                    bail!("halt");
                }
                StepResult::WaitInput => break,
                StepResult::Output => continue,
            }
        }
        let out = self
            .instnt
            .output_iter()
            .map(|c| *c as u8 as char)
            .collect();
        self.instnt.clear_output();
        Ok(out)
    }

    fn command(&mut self, input: &str) {
        let input: Vec<Int> = input.as_bytes().iter().map(|c| *c as Int).collect();
        self.instnt.push_inputs(&input);
    }

    fn move_to(&mut self, d: Direction) -> Result<Location> {
        self.command(d.to_move_cmd().as_str());
        let out = self.reponse()?;
        let loc = Location::try_from(out.as_str())?;
        self.loc = loc.clone();
        Ok(loc)
    }

    fn take(&mut self, item: &str) -> Result<()> {
        if item != "infinite loop"
            && item != "giant electromagnet"
            && item != "photons"
            && item != "molten lava"
            && item != "escape pod"
        {
            self.items.push(item.to_owned());
            self.items.sort();
            self.command(format!("take {}\n", item).as_str());
            self.reponse()?;
        }
        Ok(())
    }

    fn drop(&mut self, item: &str) -> Result<()> {
        self.command(format!("drop {}\n", item).as_str());
        self.reponse()?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let prog = Program::load_from_input("day25.txt")?;
    let mut droid = Droid::new(&prog)?;

    let mut visited: HashSet<(String, Vec<String>)> = HashSet::new();
    let mut queue: VecDeque<Droid> = VecDeque::new();
    queue.push_back(droid.clone());
    while let Some(mut cur_droid) = queue.pop_front() {
        if cur_droid.loc.name == "Security Checkpoint" && cur_droid.items.len() > droid.items.len()
        {
            droid = cur_droid.clone();
        }
        if visited.insert((cur_droid.loc.name.clone(), cur_droid.items.clone())) {
            let loc = cur_droid.loc.clone();
            for item in &loc.items {
                cur_droid.take(item)?;
            }
            for &door in &loc.doors {
                let mut d = cur_droid.clone();
                d.move_to(door)?;
                queue.push_back(d);
            }
        }
    }

    for idx in 0..(1 << droid.items.len()) {
        let mut d = droid.clone();
        for (i, item) in droid.items.iter().enumerate() {
            if idx & (1 << i) > 0 {
                d.drop(item)?;
            }
        }
        d.command("north\n");
        if let Ok(out) = d.reponse() {
            if !out.contains("Security Checkpoint") {
                println!("{}", out);
                break;
            }
        }
    }

    Ok(())
}
