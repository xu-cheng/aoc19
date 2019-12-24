use aoc2019::*;
use std::collections::{BTreeMap, HashSet};

const WIDTH: usize = 5;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
    level: i32,
}

impl Coordinate {
    fn adjacent(self, recursive: bool) -> HashSet<Self> {
        let mut ans = HashSet::new();
        if self.x >= 1 {
            ans.insert(Coordinate {
                x: self.x - 1,
                y: self.y,
                level: self.level,
            });
        }
        if self.x + 1 < WIDTH {
            ans.insert(Coordinate {
                x: self.x + 1,
                y: self.y,
                level: self.level,
            });
        }
        if self.y >= 1 {
            ans.insert(Coordinate {
                x: self.x,
                y: self.y - 1,
                level: self.level,
            });
        }
        if self.y + 1 < WIDTH {
            ans.insert(Coordinate {
                x: self.x,
                y: self.y + 1,
                level: self.level,
            });
        }
        if recursive {
            if self.x == 0 {
                ans.insert(Coordinate {
                    x: 1,
                    y: 2,
                    level: self.level - 1,
                });
            }
            if self.x == WIDTH - 1 {
                ans.insert(Coordinate {
                    x: 3,
                    y: 2,
                    level: self.level - 1,
                });
            }
            if self.y == 0 {
                ans.insert(Coordinate {
                    x: 2,
                    y: 1,
                    level: self.level - 1,
                });
            }
            if self.y == WIDTH - 1 {
                ans.insert(Coordinate {
                    x: 2,
                    y: 3,
                    level: self.level - 1,
                });
            }
            if self.x == 2 && self.y == 1 {
                for x in 0..WIDTH {
                    ans.insert(Coordinate {
                        x,
                        y: 0,
                        level: self.level + 1,
                    });
                }
            }
            if self.x == 2 && self.y == 3 {
                for x in 0..WIDTH {
                    ans.insert(Coordinate {
                        x,
                        y: WIDTH - 1,
                        level: self.level + 1,
                    });
                }
            }
            if self.x == 1 && self.y == 2 {
                for y in 0..WIDTH {
                    ans.insert(Coordinate {
                        x: 0,
                        y,
                        level: self.level + 1,
                    });
                }
            }
            if self.x == 3 && self.y == 2 {
                for y in 0..WIDTH {
                    ans.insert(Coordinate {
                        x: WIDTH - 1,
                        y,
                        level: self.level + 1,
                    });
                }
            }
            ans.remove(&Coordinate {
                x: 2,
                y: 2,
                level: self.level,
            });
        }
        ans
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Bugs {
    data: BTreeMap<i32, Vec<bool>>,
}

impl Bugs {
    fn load_from_str(input: &str) -> Self {
        let layer: Vec<bool> = input
            .replace('\n', "")
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => unreachable!(),
            })
            .collect();

        let mut data = BTreeMap::new();
        data.insert(0, layer);
        Self { data }
    }

    fn load_from_input(path: &str) -> Result<Self> {
        let mut reader = open_input(path)?;
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(Self::load_from_str(&buf))
    }

    fn get(&self, loc: Coordinate) -> bool {
        self.data
            .get(&loc.level)
            .map(|v| v[loc.x + loc.y * WIDTH])
            .unwrap_or(false)
    }

    fn set(&mut self, loc: Coordinate, val: bool) {
        *self
            .data
            .entry(loc.level)
            .or_insert_with(|| [false; WIDTH * WIDTH].to_vec())
            .get_mut(loc.x + loc.y * WIDTH)
            .unwrap() = val;
    }

    fn step(&self, recursive: bool) -> Self {
        let mut out = self.clone();
        let mut min_level = *self.data.keys().next().unwrap();
        let mut max_level = *self.data.keys().last().unwrap();

        if recursive {
            min_level -= 1;
            max_level += 1;
            out.data.insert(min_level, [false; WIDTH * WIDTH].to_vec());
            out.data.insert(max_level, [false; WIDTH * WIDTH].to_vec());
        }

        for level in min_level..=max_level {
            for y in 0..WIDTH {
                for x in 0..WIDTH {
                    if recursive && x == 2 && y == 2 {
                        continue;
                    }

                    let cur_loc = Coordinate { x, y, level };
                    let cnt = cur_loc
                        .adjacent(recursive)
                        .iter()
                        .map(|&loc| self.get(loc))
                        .filter(|&x| x)
                        .count();

                    if self.get(cur_loc) {
                        if cnt != 1 {
                            out.set(cur_loc, false);
                        }
                    } else if cnt == 1 || cnt == 2 {
                        out.set(cur_loc, true);
                    }
                }
            }
        }

        if recursive {
            if out.count_level(min_level) == 0 {
                out.data.remove(&min_level);
            }
            if out.count_level(max_level) == 0 {
                out.data.remove(&max_level);
            }
        }

        out
    }

    fn rating(&self, level: i32) -> i64 {
        let mut out: i64 = 0;
        let mut cur: i64 = 1;
        for y in 0..WIDTH {
            for x in 0..WIDTH {
                if self.get(Coordinate { x, y, level }) {
                    out += cur;
                }
                cur <<= 1;
            }
        }
        out
    }

    fn count_level(&self, level: i32) -> usize {
        let mut cnt = 0;
        for y in 0..WIDTH {
            for x in 0..WIDTH {
                let cur_loc = Coordinate { x, y, level };
                if self.get(cur_loc) {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    fn count(&self) -> usize {
        let mut cnt = 0;
        let min_level = *self.data.keys().next().unwrap();
        let max_level = *self.data.keys().last().unwrap();

        for level in min_level..=max_level {
            cnt += self.count_level(level);
        }
        cnt
    }
}

fn main() -> Result<()> {
    let bugs = Bugs::load_from_input("day24.txt")?;
    let mut seen = HashSet::new();

    let mut cur = bugs.clone();
    while seen.insert(cur.clone()) {
        cur = cur.step(false);
    }
    println!("ans1={}", cur.rating(0));

    cur = bugs;
    for _ in 0..200 {
        cur = cur.step(true);
    }
    println!("ans2={}", cur.count());

    Ok(())
}
