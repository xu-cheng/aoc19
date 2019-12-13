//! Intcode Computer used in Day 05, 07, 09, 11, 13

use crate::*;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::iter::FromIterator;

pub type Int = i64;

pub struct Program(Vec<Int>);

impl Program {
    pub fn new(code: Vec<Int>) -> Self {
        Self(code)
    }

    pub fn load_from_str(code: &str) -> Self {
        Self(
            code.trim()
                .split(',')
                .filter_map(|op| op.parse().ok())
                .collect::<Vec<Int>>(),
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
            base: 0,
            mem: self.0.clone(),
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn start_with_input(&self, input: &[Int]) -> Instant {
        Instant {
            pc: 0,
            base: 0,
            mem: self.0.clone(),
            input: VecDeque::from_iter(input.iter().cloned()),
            output: VecDeque::new(),
        }
    }
}

pub struct Instant {
    pub pc: usize,
    pub base: Int,
    pub mem: Vec<Int>,
    pub input: VecDeque<Int>,
    pub output: VecDeque<Int>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StepResult {
    Output,
    WaitInput,
    Halt,
}

enum ParameterMode {
    Immediate,
    Position,
    Relative,
}

impl TryFrom<Int> for ParameterMode {
    type Error = Error;

    fn try_from(value: Int) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            2 => Ok(ParameterMode::Relative),
            _ => bail!("invalid mode {}", value),
        }
    }
}

enum OpCode {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equal(ParameterMode, ParameterMode, ParameterMode),
    AdjustBase(ParameterMode),
    Halt,
}

impl TryFrom<Int> for OpCode {
    type Error = Error;

    fn try_from(code: Int) -> std::result::Result<Self, Self::Error> {
        let m1 = ParameterMode::try_from((code / 100) % 10)?;
        let m2 = ParameterMode::try_from((code / 1000) % 10)?;
        let m3 = ParameterMode::try_from((code / 10000) % 10)?;

        match code % 100 {
            1 => Ok(OpCode::Add(m1, m2, m3)),
            2 => Ok(OpCode::Mul(m1, m2, m3)),
            3 => Ok(OpCode::Input(m1)),
            4 => Ok(OpCode::Output(m1)),
            5 => Ok(OpCode::JumpIfTrue(m1, m2)),
            6 => Ok(OpCode::JumpIfFalse(m1, m2)),
            7 => Ok(OpCode::LessThan(m1, m2, m3)),
            8 => Ok(OpCode::Equal(m1, m2, m3)),
            9 => Ok(OpCode::AdjustBase(m1)),
            99 => Ok(OpCode::Halt),
            _ => bail!("invalid op code {}", code),
        }
    }
}

impl Instant {
    fn read(&self, addr: usize) -> Int {
        match self.mem.get(addr).copied() {
            Some(val) => val,
            None => 0,
        }
    }

    fn deref_read(&self, addr: usize, base: Int) -> Int {
        self.read((self.read(addr) + base) as usize)
    }

    fn write(&mut self, addr: usize, val: Int) {
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr] = val;
    }

    fn deref_write(&mut self, addr: usize, base: Int, val: Int) {
        self.write((self.read(addr) + base) as usize, val)
    }

    fn read_parameter(&self, idx: usize, mode: ParameterMode) -> Int {
        match mode {
            ParameterMode::Position => self.deref_read(self.pc + idx, 0),
            ParameterMode::Immediate => self.read(self.pc + idx),
            ParameterMode::Relative => self.deref_read(self.pc + idx, self.base),
        }
    }

    fn write_parameter(&mut self, idx: usize, mode: ParameterMode, val: Int) {
        match mode {
            ParameterMode::Position => self.deref_write(self.pc + idx, 0, val),
            ParameterMode::Immediate => self.write(self.pc + idx, val),
            ParameterMode::Relative => self.deref_write(self.pc + idx, self.base, val),
        }
    }

    pub fn step(&mut self) -> Result<StepResult> {
        loop {
            match OpCode::try_from(self.read(self.pc))? {
                OpCode::Add(m1, m2, m3) => {
                    let val1 = self.read_parameter(1, m1);
                    let val2 = self.read_parameter(2, m2);
                    let val3 = val1 + val2;
                    self.write_parameter(3, m3, val3);
                    self.pc += 4;
                }
                OpCode::Mul(m1, m2, m3) => {
                    let val1 = self.read_parameter(1, m1);
                    let val2 = self.read_parameter(2, m2);
                    let val3 = val1 * val2;
                    self.write_parameter(3, m3, val3);
                    self.pc += 4;
                }
                OpCode::Input(m1) => {
                    if let Some(val) = self.input.pop_front() {
                        self.write_parameter(1, m1, val);
                        self.pc += 2;
                    } else {
                        break Ok(StepResult::WaitInput);
                    }
                }
                OpCode::Output(m1) => {
                    self.output.push_back(self.read_parameter(1, m1));
                    self.pc += 2;
                    break Ok(StepResult::Output);
                }
                OpCode::JumpIfTrue(m1, m2) => {
                    let val = self.read_parameter(1, m1);
                    if val != 0 {
                        self.pc = self.read_parameter(2, m2) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                OpCode::JumpIfFalse(m1, m2) => {
                    let val = self.read_parameter(1, m1);
                    if val == 0 {
                        self.pc = self.read_parameter(2, m2) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                OpCode::LessThan(m1, m2, m3) => {
                    let val1 = self.read_parameter(1, m1);
                    let val2 = self.read_parameter(2, m2);
                    let val3 = if val1 < val2 { 1 } else { 0 };
                    self.write_parameter(3, m3, val3);
                    self.pc += 4;
                }
                OpCode::Equal(m1, m2, m3) => {
                    let val1 = self.read_parameter(1, m1);
                    let val2 = self.read_parameter(2, m2);
                    let val3 = if val1 == val2 { 1 } else { 0 };
                    self.write_parameter(3, m3, val3);
                    self.pc += 4;
                }
                OpCode::AdjustBase(m1) => {
                    self.base += self.read_parameter(1, m1);
                    self.pc += 2;
                }
                OpCode::Halt => break Ok(StepResult::Halt),
            }
        }
    }

    pub fn execute(&mut self) -> Result<Vec<Int>> {
        loop {
            match self.step()? {
                StepResult::Halt => break Ok(Vec::from(self.output.clone())),
                StepResult::Output => {}
                StepResult::WaitInput => bail!("failed to get input"),
            }
        }
    }
}
