use aoc2019::*;

fn cal_fuel(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel > 0 {
        fuel
    } else {
        0
    }
}

fn main() -> Result<()> {
    let reader = open_input("day01.txt")?;
    let inputs: Vec<i32> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse().ok())
        .collect();

    let ans1 = inputs.iter().map(|&m| cal_fuel(m)).sum::<i32>();
    println!("ans1={}", ans1);

    let ans2 = inputs
        .iter()
        .flat_map(|&m| {
            itertools::unfold(m, |last| {
                let fuel = cal_fuel(*last);
                *last = fuel;
                if fuel > 0 {
                    Some(fuel)
                } else {
                    None
                }
            })
        })
        .sum::<i32>();
    println!("ans2={}", ans2);

    Ok(())
}
