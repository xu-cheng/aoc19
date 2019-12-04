use aoc2019::*;

fn main() -> Result<()> {
    let passwds: Vec<_> = (265275..=781584)
        .map(|value| {
            itertools::unfold(value, |v| {
                if *v > 0 {
                    let x = *v % 10;
                    *v /= 10;
                    Some(x as u8)
                } else {
                    None
                }
            })
            .batching(|it| {
                let x = it.next()?;
                let count = it.take_while_ref(|y| x == *y).count() as u8;
                Some((x, count + 1))
            })
            .collect::<Vec<(u8, u8)>>()
        })
        .filter(|passwd| {
            let mut last = passwd[0].0;
            let mut flag = passwd[0].1 >= 2;
            for x in &passwd[1..] {
                flag = flag || x.1 >= 2;
                if x.0 > last {
                    return false;
                }
                last = x.0;
            }
            flag
        })
        .collect();
    println!("ans1={}", passwds.len());

    let ans2 = passwds
        .iter()
        .filter(|passwd| passwd.iter().any(|(_, c)| *c == 2))
        .count();
    println!("ans2={}", ans2);

    Ok(())
}
