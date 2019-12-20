use aoc2019::*;
use petgraph::algo::astar;
use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Label {
    Passage,
    InPortal([u8; 2]),
    OutPortal([u8; 2]),
}

impl Label {
    fn from_chars(a: char, b: char, outer: bool) -> Self {
        if outer {
            Label::OutPortal([a as u8, b as u8])
        } else {
            Label::InPortal([a as u8, b as u8])
        }
    }

    fn to_str(self) -> Option<String> {
        match self {
            Label::Passage => None,
            Label::InPortal(x) | Label::OutPortal(x) => String::from_utf8(x.to_vec()).ok(),
        }
    }

    fn is_passage(self) -> bool {
        match self {
            Label::Passage => true,
            _ => false,
        }
    }

    fn is_portal(self) -> bool {
        !self.is_passage()
    }
}

#[derive(Clone)]
struct Maze {
    g: UnGraph<Label, i32>,
    dict: HashMap<String, Vec<NodeIndex>>,
}

struct MazeInput {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl MazeInput {
    fn load_from_input(path: &str) -> Result<Self> {
        let mut reader = open_input(path)?;
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(Self::load_from_str(&buf))
    }

    fn load_from_str(input: &str) -> Self {
        let height = input.lines().count();
        let data: Vec<u8> = Vec::from(input.replace('\n', "").as_bytes());
        let width = data.len() / height;

        Self {
            data,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.data[x + y * self.width] as char
    }

    fn to_maze(&self) -> Maze {
        let mut maze = Maze {
            g: Graph::default(),
            dict: HashMap::new(),
        };
        let mut visited: HashMap<(usize, usize), _> = HashMap::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let cur = self.get(x, y);
                if cur == '#' || cur == ' ' {
                    continue;
                }

                let mut w = 1;
                let node = if cur == '.' {
                    *visited
                        .entry((x, y))
                        .or_insert_with(|| maze.g.add_node(Label::Passage))
                } else if cur.is_ascii_alphabetic() {
                    w = 0;
                    if let Some(&node) = visited.get(&(x, y)) {
                        node
                    } else {
                        let outer = x == 0 || x == self.width - 2 || y == 0 || y == self.height - 2;

                        if y + 1 < self.height && self.get(x, y + 1).is_ascii_alphabetic() {
                            let node =
                                maze.g
                                    .add_node(Label::from_chars(cur, self.get(x, y + 1), outer));
                            visited.insert((x, y), node);
                            visited.insert((x, y + 1), node);
                            node
                        } else if x + 1 < self.width && self.get(x + 1, y).is_ascii_alphabetic() {
                            let node =
                                maze.g
                                    .add_node(Label::from_chars(cur, self.get(x + 1, y), outer));
                            visited.insert((x, y), node);
                            visited.insert((x + 1, y), node);
                            node
                        } else {
                            unreachable!();
                        }
                    }
                } else {
                    unreachable!();
                };

                if let Some(&t_node) = visited.get(&(x, y - 1)) {
                    if t_node != node {
                        if maze.g.node_weight(t_node).unwrap().is_portal() {
                            w = 0;
                        };
                        maze.g.add_edge(node, t_node, w);
                    }
                }

                if let Some(&l_node) = visited.get(&(x - 1, y)) {
                    if l_node != node {
                        if maze.g.node_weight(l_node).unwrap().is_portal() {
                            w = 0;
                        };
                        maze.g.add_edge(node, l_node, w);
                    }
                }
            }
        }

        while let Some(node) = maze
            .g
            .node_references()
            .find(|(n, l)| l.is_passage() && maze.g.edges(*n).count() == 2)
        {
            let n = node.0;
            let w = maze.g.edges(n).map(|e| *e.weight()).sum();
            let neighbors: Vec<_> = maze.g.neighbors(n).collect();
            maze.g.add_edge(neighbors[0], neighbors[1], w);
            maze.g.remove_node(n);
        }

        for (node, label) in maze.g.node_references() {
            if let Some(label_name) = label.to_str() {
                maze.dict
                    .entry(label_name)
                    .or_insert_with(Vec::new)
                    .push(node);
            }
        }

        maze
    }
}

fn q1(mut maze: Maze, s: &str, e: &str) -> i32 {
    for nodes in maze.dict.values() {
        if nodes.len() == 2 {
            maze.g.add_edge(nodes[0], nodes[1], 1);
        }
    }

    let start_node = maze.dict[s][0];
    let end_node = maze.dict[e][0];
    let res = astar(
        &maze.g,
        start_node,
        |f| f == end_node,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();
    res.0
}

fn q2(maze: &Maze, s: &str, e: &str) -> i32 {
    #[derive(Hash, Clone, Copy, Eq, PartialEq)]
    struct Loc(NodeIndex, i32);

    let start = Loc(maze.dict[s][0], 0);
    let end = Loc(maze.dict[e][0], 0);

    let mut visited: HashSet<Loc> = HashSet::new();
    let mut pq = PriorityQueue::new();
    pq.push(start, Reverse(0));
    visited.insert(start);
    while !pq.is_empty() {
        let (loc, Reverse(cost)) = pq.pop().unwrap();

        if loc == end {
            return cost;
        }

        let Loc(node, level) = loc;

        for e in maze.g.edges(node) {
            let next_node = if e.source() == node {
                e.target()
            } else {
                e.source()
            };
            let next_loc = Loc(next_node, level);
            if visited.insert(next_loc) {
                pq.push(next_loc, Reverse(cost + *e.weight()));
            }
        }

        let label = maze.g.node_weight(node).unwrap();
        if let Some(label_name) = label.to_str() {
            let next_level = match &label {
                Label::InPortal(_) => level + 1,
                Label::OutPortal(_) => level - 1,
                _ => unreachable!(),
            };

            if next_level < 0 {
                continue;
            }

            if let Some(&next_node) = maze.dict[&label_name].iter().find(|n| **n != node) {
                let next_loc = Loc(next_node, next_level);

                if visited.insert(next_loc) {
                    pq.push(next_loc, Reverse(cost + 1));
                }
            }
        }
    }

    unreachable!();
}

fn main() -> Result<()> {
    let input = MazeInput::load_from_input("day20.txt")?;
    let maze = input.to_maze();

    let ans1 = q1(maze.clone(), "AA", "ZZ");
    println!("ans1={:?}", ans1);

    let ans2 = q2(&maze, "AA", "ZZ");
    println!("ans2={:?}", ans2);

    Ok(())
}
