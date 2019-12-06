extern crate petgraph;

use aoc2019::*;
use petgraph::prelude::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    let reader = open_input("day06.txt")?;
    let input: Vec<(String, String)> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.trim().split(')').map(|s| s.to_string()).next_tuple())
        .collect();

    let mut g: DiGraphMap<&str, ()> = DiGraphMap::new();
    for (a, b) in &input {
        g.add_edge(a, b, ());
    }

    let root = "COM";
    let mut depths: HashMap<&str, i32> = HashMap::new();
    depths.insert(root, 0);
    let mut bfs = Bfs::new(&g, &root);
    let mut ans1 = 0;
    while let Some(node) = bfs.next(&g) {
        let d = *depths.get(node).unwrap();
        ans1 += d;
        for succ in g.neighbors(node) {
            depths.insert(&succ, d + 1);
        }
    }
    println!("ans1={:?}", ans1);

    let mut nodes = ["YOU", "SAN"];
    let mut nodes_d = [depths[nodes[0]], depths[nodes[1]]];
    while nodes[0] != nodes[1] {
        let idx = if nodes_d[0] > nodes_d[1] { 0 } else { 1 };
        nodes[idx] = g
            .neighbors_directed(&nodes[idx], Direction::Incoming)
            .next()
            .unwrap();
        nodes_d[idx] = depths[nodes[idx]];
    }
    println!(
        "ans2={:?}",
        depths["YOU"] + depths["SAN"] - 2 * nodes_d[0] - 2
    );

    Ok(())
}
