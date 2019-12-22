#[macro_use]
extern crate lazy_static;

use aoc2019::*;
use num::Integer;
use regex::Regex;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Sub};

lazy_static! {
    static ref CUT_RE: Regex = Regex::new(r"cut (-?\d+)").unwrap();
    static ref DEAL_RE: Regex = Regex::new(r"deal with increment (\d+)").unwrap();
}

enum Input {
    Stack,
    Cut(i64),
    Deal(i64),
}

impl TryFrom<&str> for Input {
    type Error = Error;

    fn try_from(input: &str) -> Result<Input> {
        if input == "deal into new stack" {
            Ok(Input::Stack)
        } else if let Some(cap) = CUT_RE.captures(input) {
            let x = cap.get(1).unwrap().as_str().parse()?;
            Ok(Input::Cut(x))
        } else if let Some(cap) = DEAL_RE.captures(input) {
            let x = cap.get(1).unwrap().as_str().parse()?;
            Ok(Input::Deal(x))
        } else {
            bail!("invalid input");
        }
    }
}

macro_rules! impl_modular_int {
    ($x:ident, $t: ty) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        struct $x {
            num: $t,
            modular: $t,
        }

        #[allow(dead_code)]
        impl $x {
            fn new(mut num: $t, modular: $t) -> Self {
                num %= modular;
                if num < 0 {
                    num += modular;
                }
                Self { num, modular }
            }

            fn zero(self) -> Self {
                Self {
                    num: 0,
                    modular: self.modular,
                }
            }

            fn one(self) -> Self {
                Self {
                    num: 1,
                    modular: self.modular,
                }
            }

            fn inv(self) -> Self {
                Self::new(<$t>::extended_gcd(&self.num, &self.modular).x, self.modular)
            }

            fn pow(self, mut e: i64) -> Self {
                assert!(e > 0);
                let mut b = self;
                let mut r = self.one();
                while e > 0 {
                    if (e & 1) == 1 {
                        r = r * b;
                    }
                    e >>= 1;
                    b = b * b;
                }
                r
            }
        }

        impl Add for $x {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                assert_eq!(self.modular, other.modular);
                Self::new(self.num + other.num, self.modular)
            }
        }

        impl Sub for $x {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                assert_eq!(self.modular, other.modular);
                Self::new(self.num - other.num, self.modular)
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl Mul for $x {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                assert_eq!(self.modular, other.modular);
                let mut a = self;
                let mut b = other.num;
                let mut r = self.zero();
                while b > 0 {
                    if (b & 1) == 1 {
                        r = r + a;
                    }
                    b >>= 1;
                    a.num = (a.num << 1) % a.modular;
                }
                r
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl Div for $x {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                assert_eq!(self.modular, other.modular);
                self * other.inv()
            }
        }
    };
}

impl_modular_int!(ModI64, i64);
impl_modular_int!(ModI128, i128);

#[derive(Debug, Copy, Clone)]
struct Cards(ModI64);

impl Cards {
    fn new(pos: i64, len: i64) -> Self {
        Self(ModI64::new(pos, len))
    }

    fn stack(&mut self) {
        self.0.num = self.0.modular - 1 - self.0.num;
    }

    fn rev_stack(&mut self) {
        self.stack()
    }

    fn cut(&mut self, cut: i64) {
        self.0 = self.0 - ModI64::new(cut, self.0.modular);
    }

    fn rev_cut(&mut self, cut: i64) {
        self.0 = self.0 + ModI64::new(cut, self.0.modular);
    }

    fn deal(&mut self, deal: i64) {
        self.0 = self.0 * ModI64::new(deal, self.0.modular);
    }

    fn rev_deal(&mut self, deal: i64) {
        self.0 = self.0 / ModI64::new(deal, self.0.modular);
    }
}

fn main() -> Result<()> {
    let inputs: Vec<Input> = open_input("day22.txt")?
        .lines()
        .map(|l| l.map_err(Error::from))
        .map(|l| l.and_then(|l| Input::try_from(l.as_str())))
        .collect::<Result<_>>()?;

    let mut cards = Cards::new(2_019, 10_007);
    for input in &inputs {
        match input {
            Input::Stack => cards.stack(),
            Input::Cut(x) => cards.cut(*x),
            Input::Deal(x) => cards.deal(*x),
        }
    }
    println!("ans1={:?}", cards.0.num);

    let num_cards = 119_315_717_514_047i64;
    let k = 101_741_582_076_661i64;
    let cards0 = Cards::new(2_020, num_cards);
    let mut cards1 = cards0;
    for input in inputs.iter().rev() {
        match input {
            Input::Stack => cards1.rev_stack(),
            Input::Cut(x) => cards1.rev_cut(*x),
            Input::Deal(x) => cards1.rev_deal(*x),
        }
    }
    let mut cards2 = cards1;
    for input in inputs.iter().rev() {
        match input {
            Input::Stack => cards2.rev_stack(),
            Input::Cut(x) => cards2.rev_cut(*x),
            Input::Deal(x) => cards2.rev_deal(*x),
        }
    }
    // Compute a, b in f(x+1) = (f(x)*a + b) % m
    let a = (cards2.0 - cards1.0) / (cards1.0 - cards0.0);
    let b = cards1.0 - a * cards0.0;

    // f(x+k) = a^k * f(0) + [(a^k - 1) % (a-1)m] * b / (a -1)
    let a1 = a.num as i128 - 1;
    let am = a1 * num_cards as i128;
    let a_pow1 = a.pow(k);
    let a_pow2 = ModI128::new(a.num as i128, am).pow(k).num;
    let c = (a_pow2 - 1) / a1 * (b.num as i128) % (num_cards as i128);
    let cm = ModI64::new(c as i64, num_cards);
    let ans2 = cards0.0 * a_pow1 + cm;
    println!("ans2={:?}", ans2.num);

    Ok(())
}
