use aoc2019::*;
use std::cell::RefCell;

fn run(input: &[usize], non: usize, verb: usize) -> Result<usize> {
    let mut mem = Vec::from(input);
    mem[1] = non;
    mem[2] = verb;

    let mem_cell = RefCell::new(&mut mem);
    let read = |loc: usize| -> Result<usize> {
        mem_cell.borrow().get(loc).copied().context("out of range")
    };
    let deref_read = |loc: usize| -> Result<usize> { read(read(loc)?) };
    let write = |loc: usize, val: usize| -> Result<()> {
        mem_cell
            .borrow_mut()
            .get_mut(loc)
            .map(|v| *v = val)
            .context("out of range")
    };
    let mut pc: usize = 0;
    loop {
        match read(pc)? {
            1 => {
                write(read(pc + 3)?, deref_read(pc + 1)? + deref_read(pc + 2)?)?;
                pc += 4;
            }
            2 => {
                write(read(pc + 3)?, deref_read(pc + 1)? * deref_read(pc + 2)?)?;
                pc += 4;
            }
            99 => break,
            _ => bail!("invalid code"),
        }
    }
    read(0)
}

fn solve(input: &[usize], result: usize) -> Result<(usize, usize)> {
    for non in 0..100 {
        for verb in 0..100 {
            if run(input, non, verb).ok() == Some(result) {
                return Ok((non, verb));
            }
        }
    }
    bail!("failed to solve the problem!");
}

fn main() -> Result<()> {
    let mut reader = open_input("day02.txt")?;
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let input: Vec<usize> = buf
        .trim()
        .split(',')
        .filter_map(|op| op.parse().ok())
        .collect();

    println!("ans1={}", run(&input, 12, 2)?);

    let (non, verb) = solve(&input, 19_690_720)?;
    println!("ans2={}", 100 * non + verb);

    Ok(())
}
