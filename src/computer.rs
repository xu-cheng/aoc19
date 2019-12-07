//! Intcode Computer used in Day 05, 07

use crate::*;
use std::collections::VecDeque;
use std::iter::FromIterator;

pub struct Program(Vec<i32>);

impl Program {
    pub fn new(code: Vec<i32>) -> Self {
        Self(code)
    }

    pub fn load_from_str(code: &str) -> Self {
        Self(
            code.trim()
                .split(',')
                .filter_map(|op| op.parse().ok())
                .collect::<Vec<i32>>(),
        )
    }

    pub fn load_from_input(path: &str) -> Result<Self> {
        let mut reader = open_input(path)?;
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        Ok(Self::load_from_str(&buf))
    }

    pub fn start(&self) -> Instant {
        Instant {
            pc: 0,
            mem: self.0.clone(),
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn start_with_input(&self, input: &[i32]) -> Instant {
        Instant {
            pc: 0,
            mem: self.0.clone(),
            input: VecDeque::from_iter(input.iter().cloned()),
            output: VecDeque::new(),
        }
    }
}

pub struct Instant {
    pub pc: usize,
    pub mem: Vec<i32>,
    pub input: VecDeque<i32>,
    pub output: VecDeque<i32>,
}

pub enum StepResult {
    Output,
    Halt,
}

impl Instant {
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

    fn deref_write(&mut self, addr: usize, val: i32) -> Result<()> {
        self.write(self.read(addr)? as usize, val)
    }

    pub fn step(&mut self) -> Result<StepResult> {
        loop {
            let code = self.read(self.pc)?;
            let opcode = code % 100;
            match opcode {
                1 => {
                    let val1 = self.read_paramemter(code, 1)?;
                    let val2 = self.read_paramemter(code, 2)?;
                    let val3 = val1 + val2;
                    self.deref_write(self.pc + 3, val3)?;
                    self.pc += 4;
                }
                2 => {
                    let val1 = self.read_paramemter(code, 1)?;
                    let val2 = self.read_paramemter(code, 2)?;
                    let val3 = val1 * val2;
                    self.deref_write(self.pc + 3, val3)?;
                    self.pc += 4;
                }
                3 => {
                    let val = self
                        .input
                        .pop_front()
                        .ok_or_else(|| anyhow!("failed to read input"))?;
                    self.write(self.read(self.pc + 1)? as usize, val)?;
                    self.pc += 2;
                }
                4 => {
                    self.output.push_back(self.read_paramemter(code, 1)?);
                    self.pc += 2;
                    break Ok(StepResult::Output);
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
                    self.deref_write(self.pc + 3, val3)?;
                    self.pc += 4;
                }
                8 => {
                    let val1 = self.read_paramemter(code, 1)?;
                    let val2 = self.read_paramemter(code, 2)?;
                    let val3 = if val1 == val2 { 1 } else { 0 };
                    self.deref_write(self.pc + 3, val3)?;
                    self.pc += 4;
                }
                99 => break Ok(StepResult::Halt),
                _ => bail!("invalid code: {} at {}", code, self.pc),
            }
        }
    }

    pub fn execute(&mut self) -> Result<Vec<i32>> {
        loop {
            match self.step()? {
                StepResult::Halt => break Ok(Vec::from(self.output.clone())),
                StepResult::Output => {}
            }
        }
    }
}
