use aoc2019::computer::*;
use aoc2019::*;

fn main() -> Result<()> {
    let prog = Program::load_from_input("day05.txt")?;

    let ans1 = prog.start_with_input(&[1]).execute()?;
    println!("ans1={:?}", ans1);

    let ans2 = prog.start_with_input(&[5]).execute()?;
    println!("ans2={:?}", ans2);

    Ok(())
}
