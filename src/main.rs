#![allow(dead_code)]

use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

fn e1() -> &'static str {
    "\
Register A: 0
Register B: 0
Register C: 9

Program: 2,6
"
}
fn e2() -> &'static str {
    "\
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
"
}
fn e3() -> &'static str {
    "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
}
fn e4() -> &'static str {
    "\
Register A: 0
Register B: 29
Register C: 0

Program: 1,7
"
}
fn e5() -> &'static str {
    "\
Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0
"
}
fn e6() -> &'static str {
    "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
}
fn e7() -> &'static str {
    "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
}
fn d() -> &'static str {
    include_str!("input.txt")
}

const fn combo<'a>(op: &'a usize, a: &'a usize, b: &'a usize, c: &'a usize) -> &'a usize {
    match op {
        0..=3 => op,
        4 => a,
        5 => b,
        6 => c,
        7 => panic!("reserved and will not appear in valid programs"),
        _ => panic!("??"),
    }
}
fn run(
    instrs: &[usize],
    a: usize,
    b: usize,
    c: usize,
    _: &mut BTreeMap<BTreeSet<usize>, String>,
) -> (usize, usize, usize, String) {
    let mut out = String::new();
    let mut ip = 0;
    let mut a = a;
    let mut b = b;
    let mut c = c;
    loop {
        let instr = instrs[ip];
        let op = instrs[ip + 1];
        match instr {
            0 => a = a >> combo(&op, &a, &b, &c),
            1 => b ^= op,
            2 => b = combo(&op, &a, &b, &c) % 8,
            3 => (), // handling later
            4 => b ^= c,
            5 => out.push(((combo(&op, &a, &b, &c) % 8) as u8 | 0x30) as char),
            6 => b = a >> combo(&op, &a, &b, &c),
            7 => c = a >> combo(&op, &a, &b, &c),
            _ => panic!("bad instr {instr}"),
        }
        if instr == 3 && a != 0 {
            ip = op;
        } else {
            ip += 2;
        }
        if ip >= instrs.len() - 1 {
            break;
        }
    }
    (a, b, c, out.chars().join(","))
}
#[derive(Debug)]
struct RegProg {
    a: usize,
    b: usize,
    c: usize,
    instrs: Vec<usize>,
}
impl RegProg {
    fn new(d: &str) -> Self {
        let vals = d
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.split(": ").nth(1).expect("vals"))
            .collect_vec();
        let regs = vals[0..3]
            .iter()
            .map(|s| s.parse::<usize>().expect("regs"))
            .collect_vec();
        let instrs = vals[3]
            .split(",")
            .map(|s| s.parse::<usize>().expect("prog"))
            .collect_vec();
        Self {
            a: regs[0],
            b: regs[1],
            c: regs[2],
            instrs,
        }
    }
    fn run(&self, cache: &mut BTreeMap<BTreeSet<usize>, String>) -> (usize, usize, usize, String) {
        run(&self.instrs, self.a, self.b, self.c, cache)
    }
}
fn main() {
    let mut cache = BTreeMap::new();
    println!("{:?}", RegProg::new(d()).run(&mut cache));
}

#[cfg(test)] #[rustfmt::skip]
mod test {
    use super::*;
    #[test] fn p1_e1() { assert_eq!((0, 1, 9, "".to_string()), RegProg::new(e1()).run(&mut BTreeMap::new())); }
    #[test] fn p1_e2() { assert_eq!((10, 0, 0, "0,1,2".to_string()), RegProg::new(e2()).run(&mut BTreeMap::new())); }
    #[test] fn p1_e3() { assert_eq!((0, 0, 0, "4,2,5,6,7,7,7,7,3,1,0".to_string()), RegProg::new(e3()).run(&mut BTreeMap::new())); }
    #[test] fn p1_e4() { assert_eq!((0, 26, 0, "".to_string()), RegProg::new(e4()).run(&mut BTreeMap::new())); }
    #[test] fn p1_e5() { assert_eq!((0, 44354, 43690, "".to_string()), RegProg::new(e5()).run(&mut BTreeMap::new())); }
    #[test] fn p1_e6() { assert_eq!((0, 0, 0, "4,6,3,5,6,3,5,2,1,0".to_string()), RegProg::new(e6()).run(&mut BTreeMap::new())); }
    #[test] fn p1_d() { assert_eq!((0, 1, 0, "6,5,4,7,1,6,0,3,1".to_string()), RegProg::new(d()).run(&mut BTreeMap::new())); }
    // #[test] fn p2_e7() { let (a, b) = run_override_2str(e7(), 117440); assert_eq!(a, b); }
    // #[test] fn p2_e7_2() { assert_eq!(117440, find_override_linear(e7())); }
}
