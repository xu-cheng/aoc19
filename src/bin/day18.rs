#![allow(clippy::type_complexity)]

use aoc2019::*;
use petgraph::algo::astar;
use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn load_from_str(input: &str, q2: bool) -> Self {
        let height = input.lines().count();
        let data: Vec<u8> = Vec::from(input.replace('\n', "").as_bytes());
        let width = data.len() / height;

        let mut map = Self {
            data,
            width,
            height,
        };

        if q2 {
            let idx = map.data.iter().position(|&c| c as char == '@').unwrap();
            let x = idx % map.width;
            let y = idx / map.width;
            map.set(x, y, '#');
            map.set(x + 1, y, '#');
            map.set(x - 1, y, '#');
            map.set(x, y + 1, '#');
            map.set(x, y - 1, '#');
            map.set(x + 1, y + 1, '!');
            map.set(x + 1, y - 1, '@');
            map.set(x - 1, y + 1, '$');
            map.set(x - 1, y - 1, '%');
        }
        map
    }

    fn load_from_input(path: &str, q2: bool) -> Result<Self> {
        let mut reader = open_input(path)?;
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(Self::load_from_str(&buf, q2))
    }

    fn set(&mut self, x: usize, y: usize, v: char) {
        self.data[x + y * self.width] = v as u8;
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.data[x + y * self.width] as char
    }

    fn to_graph(&self) -> (UnGraph<(char, i32, i32), i32>, HashMap<char, NodeIndex>) {
        let mut graph: UnGraph<(char, i32, i32), i32> = Graph::default();
        let mut visited: HashMap<(usize, usize), _> = HashMap::new();
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let c_label = self.get(x, y);
                if c_label == '#' {
                    continue;
                }

                let c_node = *visited
                    .entry((x, y))
                    .or_insert_with(|| graph.add_node((c_label, x as i32, y as i32)));

                if let Some(t_node) = visited.get(&(x, y - 1)) {
                    graph.add_edge(c_node, *t_node, 1);
                }

                if let Some(l_node) = visited.get(&(x - 1, y)) {
                    graph.add_edge(c_node, *l_node, 1);
                }
            }
        }

        while let Some(node) = graph
            .node_references()
            .find(|(n, l)| l.0 == '.' && graph.edges(*n).count() == 2)
        {
            let n = node.0;
            let w = graph.edges(n).map(|e| *e.weight()).sum();
            let neighbors: Vec<_> = graph.neighbors(n).collect();
            graph.add_edge(neighbors[0], neighbors[1], w);
            graph.remove_node(n);
        }

        let mut dict: HashMap<char, _> = HashMap::new();
        for (node, &label) in graph.node_references() {
            if label.0 != '.' {
                dict.insert(label.0, node);
            }
        }

        (graph, dict)
    }
}

// Return (start, end) -> (cost, doors)
fn shortest_paths(
    g: &UnGraph<(char, i32, i32), i32>,
    dict: &HashMap<char, NodeIndex>,
    keys: &[char],
    starts: &[char],
) -> HashMap<(char, char), (i32, Vec<char>)> {
    let mut pathmap = HashMap::new();
    for start in keys.iter().chain(starts.iter()) {
        for end in keys.iter() {
            if start == end {
                continue;
            }
            let s_node = dict[start];
            let e_node = dict[end];
            let (_, x0, y0) = g.node_weight(s_node).unwrap();
            let astar_res = astar(
                g,
                s_node,
                |f| f == e_node,
                |e| *e.weight(),
                |n| {
                    let (_, x1, y1) = g.node_weight(n).unwrap();
                    (x1 - x0).abs() + (y1 - y0).abs()
                },
            );
            if let Some((cost, path)) = astar_res {
                let doors: Vec<char> = path
                    .iter()
                    .map(|n| g.node_weight(*n).unwrap().0)
                    .filter(|c| c.is_uppercase())
                    .collect();
                pathmap.insert((*start, *end), (cost, doors));
            }
        }
    }
    pathmap
}

// Return (start_idx, key, cost)
fn reacheable_keys(
    pathmap: &HashMap<(char, char), (i32, Vec<char>)>,
    all_keys: &[char],
    owned_keys: &[char],
    starts: &[char],
) -> Vec<(usize, char, i32)> {
    let mut ans: Vec<(usize, char, i32)> = Vec::new();
    for (i, &s) in starts.iter().enumerate() {
        for &k in all_keys {
            if s == k || owned_keys.contains(&k) {
                continue;
            }
            if let Some((cost, doors)) = pathmap.get(&(s, k)) {
                if doors
                    .iter()
                    .all(|d| owned_keys.contains(&d.to_ascii_lowercase()))
                {
                    ans.push((i, k, *cost));
                }
            }
        }
    }
    ans
}

fn min_walk(
    pathmap: &HashMap<(char, char), (i32, Vec<char>)>,
    all_keys: &[char],
    starts: &[char],
) -> i32 {
    // cache: (robots locs, obtained keys) -> cost
    fn inner(
        cache: Rc<RefCell<HashMap<(Vec<char>, Vec<char>), i32>>>,
        pathmap: &HashMap<(char, char), (i32, Vec<char>)>,
        all_keys: &[char],
        starts: Vec<char>,
        owned_keys: Vec<char>,
    ) -> i32 {
        let cache_key = (starts.clone(), owned_keys.clone());
        if let Some(x) = cache.borrow().get(&cache_key) {
            return *x;
        }
        let reachables = reacheable_keys(&pathmap, &all_keys, &owned_keys, &starts);
        let res = if reachables.is_empty() {
            0
        } else {
            reachables
                .into_iter()
                .map(|(i, k, c)| {
                    let mut n_starts = starts.clone();
                    n_starts[i] = k;
                    let mut n_keys = owned_keys.clone();
                    n_keys.push(k);
                    n_keys.sort();
                    c + inner(cache.clone(), &pathmap, &all_keys, n_starts, n_keys)
                })
                .min()
                .unwrap()
        };
        cache.borrow_mut().insert(cache_key, res);
        res
    };
    inner(
        Rc::new(RefCell::new(HashMap::new())),
        &pathmap,
        &all_keys,
        Vec::from(starts),
        Vec::new(),
    )
}

fn main() -> Result<()> {
    let map = Map::load_from_input("day18.txt", false)?;
    let (graph, dict) = map.to_graph();
    let keys: Vec<char> = dict
        .keys()
        .filter(|&&c| c != '.' && c != '@' && c.is_lowercase())
        .cloned()
        .collect();
    let pathmap = shortest_paths(&graph, &dict, &keys, &['@']);
    let cost = min_walk(&pathmap, &keys, &['@']);
    println!("ans1={:?}", cost);

    let map2 = Map::load_from_input("day18.txt", true)?;
    let starts2 = ['!', '@', '$', '%'];
    let (graph2, dict2) = map2.to_graph();
    let pathmap2 = shortest_paths(&graph2, &dict2, &keys, &starts2);
    let cost2 = min_walk(&pathmap2, &keys, &starts2);
    println!("ans2={:?}", cost2);

    Ok(())
}
