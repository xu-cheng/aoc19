use aoc2019::*;
use num_integer::lcm;
use regex::Regex;
use std::cmp::Ordering;
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Deref, DerefMut};

type Vec3 = [i32; 3];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Object {
    pos: Vec3,
    vel: Vec3,
}

impl Object {
    fn pot(&self) -> i32 {
        self.pos.iter().map(|x| x.abs()).sum()
    }

    fn kin(&self) -> i32 {
        self.vel.iter().map(|x| x.abs()).sum()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Objects(Vec<Object>);

impl Objects {
    fn pair_wise(&mut self, f: impl Fn(&mut Object, &mut Object)) {
        for i in 0..self.len() {
            for j in (i + 1)..self.len() {
                unsafe {
                    let a = &mut *(self.get_unchecked_mut(i) as *mut _);
                    let b = &mut *(self.get_unchecked_mut(j) as *mut _);
                    f(a, b);
                }
            }
        }
    }

    fn apply_gravity(&mut self) {
        self.pair_wise(|obj1, obj2| {
            for i in 0..3 {
                match obj1.pos[i].cmp(&obj2.pos[i]) {
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        obj1.vel[i] -= 1;
                        obj2.vel[i] += 1;
                    }
                    Ordering::Less => {
                        obj1.vel[i] += 1;
                        obj2.vel[i] -= 1;
                    }
                }
            }
        });
    }

    fn apply_velocity(&mut self) {
        for obj in self.iter_mut() {
            for i in 0..3 {
                obj.pos[i] += obj.vel[i];
            }
        }
    }

    fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
    }

    fn total_energy(&self) -> i32 {
        self.iter().map(|obj| obj.pot() * obj.kin()).sum()
    }
}

impl Deref for Objects {
    type Target = Vec<Object>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Objects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<Object> for Objects {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Object>,
    {
        Self(iter.into_iter().collect::<Vec<Object>>())
    }
}

fn main() -> Result<()> {
    let re = Regex::new(r#"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>"#)?;
    let reader = open_input("day12.txt")?;
    let input_objs: Objects = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            let cap = re.captures(&l)?;
            let x = cap.get(1)?.as_str().parse().ok()?;
            let y = cap.get(2)?.as_str().parse().ok()?;
            let z = cap.get(3)?.as_str().parse().ok()?;
            Some([x, y, z])
        })
        .map(|pos| Object { pos, vel: [0; 3] })
        .collect();

    let mut objs = input_objs.clone();
    for _ in 0..1000 {
        objs.step();
    }
    println!("ans1={:?}", objs.total_energy());

    let mut cnt = 0;
    let mut cycles: [Option<usize>; 3] = [None; 3];
    let mut objs = input_objs.clone();
    loop {
        cnt += 1;
        objs.step();

        for (i, c) in cycles.iter_mut().enumerate() {
            if c.is_none()
                && objs
                    .iter()
                    .zip(input_objs.iter())
                    .all(|(o1, o2)| o1.pos[i] == o2.pos[i] && o1.vel[i] == o2.vel[i])
            {
                *c = Some(cnt);
            }
        }

        if cycles.iter().all(|c| c.is_some()) {
            break;
        }
    }
    let ans2 = cycles.iter().fold(1, |res, x| lcm(res, x.unwrap()));
    println!("ans2={:?}", ans2);

    Ok(())
}
