use aoc2019::computer::*;
use aoc2019::*;

fn run_q1(prog: &Program, settings: &[Int]) -> Result<Int> {
    let mut val = 0;
    for &setting in settings {
        let input = [setting, val];
        val = prog.start_with_input(&input).execute()?[0];
    }
    Ok(val)
}

fn run_q2(prog: &Program, settings: &[Int]) -> Result<Int> {
    let mut val = 0;
    let mut instants: Vec<Instant> = Vec::with_capacity(5);
    for &setting in settings {
        instants.push(prog.start_with_input(&[setting]));
    }
    for i in (0..settings.len()).cycle() {
        let instant = &mut instants[i];
        instant.push_input(val);
        instant.step()?;
        if let Some(out) = instant.pop_output() {
            val = out;
        } else {
            break;
        }
    }
    Ok(val)
}

fn main() -> Result<()> {
    let prog = Program::load_from_input("day07.txt")?;

    let ans1 = (0..5)
        .permutations(5)
        .filter_map(|settings| run_q1(&prog, &settings).ok())
        .max()
        .unwrap();
    println!("ans1={:?}", ans1);

    let ans2 = (5..10)
        .permutations(5)
        .filter_map(|settings| run_q2(&prog, &settings).ok())
        .max()
        .unwrap();
    println!("ans2={:?}", ans2);

    Ok(())
}
