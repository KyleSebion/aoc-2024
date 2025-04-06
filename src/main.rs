#![allow(dead_code)]

use itertools::Itertools;
use std::time::Instant;

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
fn run(instrs: &[usize], a: usize, b: usize, c: usize) -> (usize, usize, usize, String) {
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
    fn run(&self) -> (usize, usize, usize, String) {
        run(&self.instrs, self.a, self.b, self.c)
    }
}
fn input_manual_jit_run(a: usize) -> (usize, usize, usize, String) {
    let mut out = String::new();
    let mut a = a;
    let mut b;
    loop {
        b = a & 0b111 ^ 0b011 ^ (a >> (a & 0b111 ^ 0b101));
        out.push(((b & 0b111) as u8 | 0x30) as char);
        a >>= 3;
        if a == 0 {
            break;
        }
    }
    (a, b, 0, out.chars().join(","))
}
fn testing_theory(mag: usize) {
    const POW: usize = 8;
    const BITS: usize = POW.trailing_zeros() as usize;
    let mut rp = RegProg::new(d());
    rp.b = 0;
    rp.c = 0;
    let mut full_cache = Vec::new();
    let mut char_cache: Vec<Vec<char>> = Vec::new(); // why do i need to annotate this!?
    let mut bads = Vec::new();
    for c_mag in 1..=mag {
        let p_mag = c_mag - 1;
        let mut sub_f_cache = Vec::new();
        let mut sub_c_cache = Vec::new();
        let p_pow = POW.pow(p_mag as u32);
        let c_pow = POW.pow(c_mag as u32);
        for i in p_pow..c_pow {
            rp.a = i;
            let i_res = rp.run().3;
            let mut c_res = i_res.chars();
            sub_c_cache.push(c_res.next().unwrap());
            if let Some(check_char) = c_res.nth(1) {
                let expect_char = char_cache[p_mag - 1][(i - p_pow) / POW];
                if expect_char != check_char {
                    bads.push(format!("{i}: {i_res} {} != {}", check_char, expect_char));
                }
            }
            sub_f_cache.push(i_res);
        }
        char_cache.push(sub_c_cache);
        full_cache.push(sub_f_cache);
    }
    if !bads.is_empty() {
        panic!("failed; found bads: {bads:?}");
    }
    println!("bads {bads:?}");
}
fn get_reg_a_for_matching_prog(d: &str) -> Option<usize> {
    const BASE: usize = 8;
    const BITS: usize = BASE.trailing_zeros() as usize;
    let mut last_pres = vec![0];
    let mut rp = RegProg::new(d);
    rp.b = 0;
    rp.c = 0;
    for pow in 0..rp.instrs.len() {
        let mut curr_pres = Vec::new();
        let need = rp.instrs.iter().nth_back(pow)?.to_string();
        for pre in last_pres {
            for suf in 0..BASE {
                let reg_a = (pre << BITS) | suf;
                rp.a = reg_a;
                let res = rp.run().3;
                // let res = input_manual_jit_run(reg_a).3; // ~50% faster (~1.5 ms to ~1 ms)
                let first = res.chars().next()?.to_string();
                if first == need {
                    curr_pres.push(reg_a);
                }
            }
        }
        if curr_pres.is_empty() {
            return None;
        } else {
            last_pres = curr_pres;
        }
    }
    last_pres.into_iter().next()
}
fn main() {
    println!("START");
    let s = Instant::now();
    println!("{:?}", get_reg_a_for_matching_prog(d()));
    println!("END {:?}", s.elapsed());
}

#[cfg(test)] #[rustfmt::skip]
mod test {
    use super::*;
    #[test] fn p1_e1() { assert_eq!(RegProg::new(e1()).run(), (0, 1, 9, "".to_string())); }
    #[test] fn p1_e2() { assert_eq!(RegProg::new(e2()).run(), (10, 0, 0, "0,1,2".to_string())); }
    #[test] fn p1_e3() { assert_eq!(RegProg::new(e3()).run(), (0, 0, 0, "4,2,5,6,7,7,7,7,3,1,0".to_string())); }
    #[test] fn p1_e4() { assert_eq!(RegProg::new(e4()).run(), (0, 26, 0, "".to_string())); }
    #[test] fn p1_e5() { assert_eq!(RegProg::new(e5()).run(), (0, 44354, 43690, "".to_string())); }
    #[test] fn p1_e6() { assert_eq!(RegProg::new(e6()).run(), (0, 0, 0, "4,6,3,5,6,3,5,2,1,0".to_string())); }
    #[test] fn p1_d()  { assert_eq!(RegProg::new( d()).run(), (0, 1, 0, "6,5,4,7,1,6,0,3,1".to_string())); }
    #[test] fn p1_d_man() { assert_eq!(input_manual_jit_run(44348299), (0, 1, 0, "6,5,4,7,1,6,0,3,1".to_string())); }
    #[test] fn p2_d() { assert_eq!(get_reg_a_for_matching_prog(d()), Some(106086382266778)); }
}
