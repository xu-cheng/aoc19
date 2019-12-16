use aoc2019::*;

const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

fn fft(input: &mut [i32]) {
    for i in 0..input.len() {
        let pattern = BASE_PATTERN
            .iter()
            .cycle()
            .flat_map(|x| itertools::repeat_n(x, i + 1))
            .skip(1 + i);
        let v: i32 = pattern.zip(input.iter().skip(i)).map(|(a, b)| a * b).sum();
        input[i] = v.abs() % 10;
    }
}

fn main() -> Result<()> {
    let mut reader = open_input("day16.txt")?;
    let mut input_str = String::new();
    reader.read_to_string(&mut input_str)?;
    let input: Vec<i32> = input_str
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|v| v as i32)
        .collect();

    let mut data1 = input.clone();
    for _ in 0..100 {
        fft(&mut data1);
    }
    println!("ans1={}", data1[..8].iter().fold(0, |res, v| res * 10 + v));

    let offset: usize = input_str[..7].parse()?;
    assert!(offset > input.len() * 10_000 / 2);
    let mut data2: Vec<i32> = input
        .iter()
        .cycle()
        .take(input.len() * 10_000)
        .skip(offset)
        .cloned()
        .collect();
    for _ in 0..100 {
        let mut sum: i32 = data2.iter().sum();
        for v in &mut data2 {
            let tmp = sum;
            sum -= *v;
            *v = tmp.abs() % 10;
        }
    }
    println!("ans2={}", data2[..8].iter().fold(0, |res, v| res * 10 + v));

    Ok(())
}
