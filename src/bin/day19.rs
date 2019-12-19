use aoc2019::computer::*;
use aoc2019::*;

fn get(prog: &Program, x: i32, y: i32) -> bool {
    let out = prog
        .start_with_input(&[x as Int, y as Int])
        .execute()
        .unwrap();
    out[0] == 1
}

fn check(prog: &Program, x: i32, y: i32, side_len: i32) -> bool {
    get(&prog, x + side_len - 1, y) && get(&prog, x, y + side_len - 1)
}

fn main() -> Result<()> {
    let prog = Program::load_from_input("day19.txt")?;

    let mut ans1 = 0;
    for x in 0..50 {
        for y in 0..50 {
            if get(&prog, x, y) {
                ans1 += 1;
            }
        }
    }
    println!("ans1={:?}", ans1);

    let side_len = 100;
    let offset = side_len - 1;
    let mut x = 0;
    let mut y = offset;
    loop {
        x = (x..).find(|&x| get(&prog, x, y)).unwrap();
        if get(&prog, x + offset, y - offset) {
            break;
        }
        y += 1;
    }
    y -= offset;

    assert!(check(&prog, x, y, side_len));
    println!("ans2={}", x * 10_000 + y);

    Ok(())
}
