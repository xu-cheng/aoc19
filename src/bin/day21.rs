use aoc2019::computer::*;
use aoc2019::*;

fn run(prog: &Program, input: &str, echo: bool) -> Result<Int> {
    let input: Vec<_> = input
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| format!("{}\n", l))
        .join("")
        .as_bytes()
        .iter()
        .map(|&b| b as Int)
        .collect();
    let mut instnt = prog.start_with_input(&input);

    let out = instnt.execute()?;
    if echo {
        for &v in &out {
            print!("{}", v as u8 as char);
        }
    }

    Ok(*out.last().unwrap())
}

fn main() -> Result<()> {
    let prog = Program::load_from_input("day21.txt")?;

    let q1 = "
        OR  A J
        AND B J
        AND C J
        NOT J J
        AND D J
        WALK
    ";
    let ans1 = run(&prog, q1, false)?;
    println!("ans1={}", ans1);

    let q2 = "
        NOT H J
        OR  C J
        AND B J
        AND A J
        NOT J J
        AND D J
        RUN
    ";
    let ans2 = run(&prog, q2, false)?;
    println!("ans2={}", ans2);

    Ok(())
}
