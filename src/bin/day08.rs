extern crate bytecount;

use aoc2019::*;

fn main() -> Result<()> {
    let mut reader = open_input("day08.txt")?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let inputs: Vec<u8> = buf
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect();

    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    const PIXELS_PER_LAYER: usize = WIDTH * HEIGHT;
    let layers: Vec<Vec<u8>> = inputs
        .into_iter()
        .chunks(PIXELS_PER_LAYER)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<u8>>())
        .collect();

    let layer = layers
        .iter()
        .min_by_key(|layer| bytecount::count(layer, 0))
        .unwrap();
    let ans1 = bytecount::count(layer, 1) * bytecount::count(layer, 2);
    println!("ans1={}", ans1);

    let image = layers
        .into_iter()
        .rev()
        .fold(vec![2 as u8; PIXELS_PER_LAYER], |img, layer| {
            img.into_iter()
                .zip(layer.into_iter())
                .map(|(p1, p2)| if p2 == 2 { p1 } else { p2 })
                .collect()
        });
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let idx = i * WIDTH + j;
            let p = image[idx];
            print!("{}", if p == 0 { "\x1B[40m " } else { "\x1B[107m " });
        }
        println!("\x1B[m");
    }

    Ok(())
}
