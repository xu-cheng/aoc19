use aoc2019::*;
use num::Integer;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
struct Component {
    name: String,
    amount: i64,
}

impl TryFrom<&str> for Component {
    type Error = Error;

    fn try_from(s: &str) -> Result<Component> {
        let mut input = s.trim().split(' ');
        let amount = input.next().context("failed to parse component")?.parse()?;
        let name = input
            .next()
            .context("failed to parse component")?
            .to_owned();
        Ok(Component { name, amount })
    }
}

type Reactions = HashMap<String, (i64, Vec<Component>)>;

fn parse_reactions(input: &str) -> Result<Reactions> {
    let mut reactions: Reactions = HashMap::new();
    for line in open_input(input)?.lines().filter_map(|l| l.ok()) {
        let mut reaction = line.split("=>");
        let ingredients = reaction
            .next()
            .context("failed to parse reaction")?
            .split(',')
            .map(|s| Component::try_from(s))
            .collect::<Result<Vec<_>>>()?;
        let product = Component::try_from(reaction.next().context("failed to parse reaction")?)?;
        reactions.insert(product.name, (product.amount, ingredients));
    }
    Ok(reactions)
}

fn ores_needed(reactions: &Reactions, fuel: i64) -> i64 {
    let mut needed: HashMap<String, i64> = HashMap::new();
    let mut surplus: HashMap<String, i64> = HashMap::new();
    let mut orces: i64 = 0;
    needed.insert("FUEL".to_owned(), fuel);
    while !needed.is_empty() {
        let chemical = needed.keys().next().unwrap().to_owned();
        let amount = needed.remove(&chemical).unwrap();
        let (units, ingredients) = reactions.get(&chemical).unwrap();
        let multiple = amount.div_ceil(&units);
        *surplus.entry(chemical.clone()).or_insert(0) += multiple * units - amount;
        for component in ingredients {
            let mut needed_units = component.amount * multiple;
            let surplus_units = surplus.entry(component.name.clone()).or_insert(0);
            let surplus_used_units = std::cmp::min(needed_units, *surplus_units);
            needed_units -= surplus_used_units;
            *surplus_units -= surplus_used_units;
            if component.name == "ORE" {
                orces += needed_units;
            } else {
                *needed.entry(component.name.clone()).or_insert(0) += needed_units;
            }
        }
    }
    orces
}

fn binary_search(mut lo: i64, mut hi: i64, f: impl Fn(i64) -> bool) -> i64 {
    while hi - lo > 1 {
        let val = (lo >> 1) + (hi >> 1) + (lo & hi & 1);
        if f(val) {
            hi = val;
        } else {
            lo = val;
        }
    }
    lo
}

fn main() -> Result<()> {
    let reactions = parse_reactions("day14.txt")?;

    println!("ans1={:?}", ores_needed(&reactions, 1));

    let ans2 = binary_search(0, 1_000_000_000, |fuel| {
        ores_needed(&reactions, fuel) > 1_000_000_000_000
    });
    println!("ans2={:?}", ans2);

    Ok(())
}
