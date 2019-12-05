use aoc2019::*;

struct Program {
    pc: usize,
    mem: Vec<i32>,
}

impl Program {
    fn new(mem: &[i32]) -> Self {
        Program {
            pc: 0,
            mem: Vec::from(mem),
        }
    }

    fn run(&mut self, mut input: impl Iterator<Item = i32>) -> Result<Vec<i32>> {
        let mut output = Vec::new();
        loop {
            let code = self.read(self.pc)?;
            let opcode = code % 100;
            match opcode {
                1 | 2 => {
                    let val1 = self.read_paramemter(code, 1)?;
                    let val2 = self.read_paramemter(code, 2)?;
                    let val3 = match opcode {
                        1 => val1 + val2,
                        2 => val1 * val2,
                        _ => unreachable!(),
                    };
                    self.write(self.read(self.pc + 3)? as usize, val3)?;
                    self.pc += 4;
                }
                3 => {
                    let val = input
                        .next()
                        .ok_or_else(|| anyhow!("failed to read input"))?;
                    self.write(self.read(self.pc + 1)? as usize, val)?;
                    self.pc += 2;
                }
                4 => {
                    let val = self.read_paramemter(code, 1)?;
                    output.push(val);
                    self.pc += 2;
                }
                5 => {
                    let val = self.read_paramemter(code, 1)?;
                    if val != 0 {
                        self.pc = self.read_paramemter(code, 2)? as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    let val = self.read_paramemter(code, 1)?;
                    if val == 0 {
                        self.pc = self.read_paramemter(code, 2)? as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                7 => {
                    let val1 = self.read_paramemter(code, 1)?;
                    let val2 = self.read_paramemter(code, 2)?;
                    let val3 = if val1 < val2 { 1 } else { 0 };
                    self.write(self.read(self.pc + 3)? as usize, val3)?;
                    self.pc += 4;
                }
                8 => {
                    let val1 = self.read_paramemter(code, 1)?;
                    let val2 = self.read_paramemter(code, 2)?;
                    let val3 = if val1 == val2 { 1 } else { 0 };
                    self.write(self.read(self.pc + 3)? as usize, val3)?;
                    self.pc += 4;
                }
                99 => break,
                _ => bail!("invalid code: {} at {}", code, self.pc),
            }
        }
        Ok(output)
    }

    fn read(&self, addr: usize) -> Result<i32> {
        self.mem
            .get(addr)
            .copied()
            .ok_or_else(|| anyhow!("out of range"))
    }

    fn deref_read(&self, addr: usize) -> Result<i32> {
        self.read(self.read(addr)? as usize)
    }

    fn read_paramemter(&self, mut code: i32, idx: usize) -> Result<i32> {
        code /= 100;
        for _ in 0..(idx - 1) {
            code /= 10;
        }
        match code % 10 {
            0 => self.deref_read(self.pc + idx),
            1 => self.read(self.pc + idx),
            _ => bail!("invalid code"),
        }
    }

    fn write(&mut self, addr: usize, val: i32) -> Result<()> {
        self.mem
            .get_mut(addr)
            .map(|v| *v = val)
            .ok_or_else(|| anyhow!("out of range"))
    }
}

fn main() -> Result<()> {
    let mut reader = open_input("day05.txt")?;
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let codes: Vec<i32> = buf
        .trim()
        .split(',')
        .filter_map(|op| op.parse().ok())
        .collect();

    let mut prog = Program::new(&codes);
    let input = vec![1].into_iter();
    println!("ans1={:?}", prog.run(input)?);

    let mut prog = Program::new(&codes);
    let input = vec![5].into_iter();
    println!("ans2={:?}", prog.run(input)?);

    Ok(())
}
