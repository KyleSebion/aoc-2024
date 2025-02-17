#![allow(dead_code)]

use std::{cmp::Ordering, time::Instant};

// B 1
fn e1() -> &'static str {
    "\
Register A: 0
Register B: 0
Register C: 9

Program: 2,6
"
}

// out 0,1,2
fn e2() -> &'static str {
    "\
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
"
}

// out 4,2,5,6,7,7,7,7,3,1,0
// A 0
fn e3() -> &'static str {
    "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
}

// B 26
fn e4() -> &'static str {
    "\
Register A: 0
Register B: 29
Register C: 0

Program: 1,7
"
}

// B 44354
fn e5() -> &'static str {
    "\
Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0
"
}

// out 4,6,3,5,6,3,5,2,1,0
fn e6() -> &'static str {
    "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
}

fn d() -> &'static str {
    "\
Register A: 44348299
Register B: 0
Register C: 0

Program: 2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0
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

const fn combo<'a>(
    op: &'a mut usize,
    a: &'a mut usize,
    b: &'a mut usize,
    c: &'a mut usize,
) -> &'a mut usize {
    match op {
        0..=3 => op,
        4 => a,
        5 => b,
        6 => c,
        7 => panic!("reserved and will not appear in valid programs"),
        _ => panic!("??"),
    }
}
type Instr = fn(
    op: &mut usize,
    a: &mut usize,
    b: &mut usize,
    c: &mut usize,
    ip: &mut usize,
    out: &mut String,
);
const INSTR_ADV: usize = 0;
const INSTR_BXL: usize = 1;
const INSTR_BST: usize = 2;
const INSTR_JNZ: usize = 3;
const INSTR_BXC: usize = 4;
const INSTR_OUT: usize = 5;
const INSTR_BDV: usize = 6;
const INSTR_CDV: usize = 7;
const INSTR_TBL: [Instr; 8] = [
    // The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
    // The denominator is found by raising 2 to the power of the instruction's combo operand.
    // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
    // The result of the division operation is truncated to an integer and then written to the A register.
    |op, a, b, c, ip, _| {
        *a /= 2_usize.pow(*combo(op, a, b, c) as u32);
        *ip += 2;
    },
    // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
    |op, _, b, _, ip, _| {
        *b ^= *op;
        *ip += 2;
    },
    // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    |op, a, b, c, ip, _| {
        *b = *combo(op, a, b, c) % 8;
        *ip += 2;
    },
    // The jnz instruction (opcode 3) does nothing if the A register is 0.
    // However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
    // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
    |op, a, _, _, ip, _| {
        if *a == 0 {
            *ip += 2;
        } else {
            *ip = *op;
        }
    },
    // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B.
    // (For legacy reasons, this instruction reads an operand but ignores it.)
    |_, _, b, c, ip, _| {
        *b ^= *c;
        *ip += 2;
    },
    // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
    // (If a program outputs multiple values, they are separated by commas.)
    |op, a, b, c, ip, out| {
        let to_out = *combo(op, a, b, c) % 8;
        if !out.is_empty() {
            out.push(',');
        }
        out.push_str(&to_out.to_string());
        *ip += 2;
    },
    // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
    // (The numerator is still read from the A register.)
    |op, a, b, c, ip, _| {
        *b = *a / 2_usize.pow(*combo(op, a, b, c) as u32);
        *ip += 2;
    },
    // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
    // (The numerator is still read from the A register.)
    |op, a, b, c, ip, _| {
        *c = *a / 2_usize.pow(*combo(op, a, b, c) as u32);
        *ip += 2;
    },
];
fn get_prog_str(d: &str) -> &str {
    get_prog_str_from_block1(d.split("\n\n").last().expect("block1"))
}
fn get_prog_str_from_block1(block1: &str) -> &str {
    block1
        .lines()
        .last()
        .expect("prog 1")
        .split(' ')
        .last()
        .expect("prog 2")
}
fn run_program_and_test(d: &str, ov_a: Option<usize>, ex_a: Option<usize>, ex_b: Option<usize>, ex_c: Option<usize>) -> String {
    let a = &mut 0;
    let b = &mut 0;
    let c = &mut 0;
    let ip = &mut 0;
    let mut out = String::new();
    let blocks = d.split("\n\n").collect::<Vec<_>>();
    for rl in blocks[0].lines() {
        let rv = rl.split([' ', ':']).collect::<Vec<_>>();
        let v = rv[3].parse().expect("reg");
        match rv[1] {
            "A" => *a = v,
            "B" => *b = v,
            "C" => *c = v,
            _ => panic!("invalid reg"),
        }
    }
    if let Some(ov_a) = ov_a {
        *a = ov_a;
    }
    let prog_str = get_prog_str_from_block1(blocks[1]);
    let prog = prog_str
        .split(',')
        // .inspect(|v| println!("{v}"))
        .map(|v| v.parse::<usize>().expect("instr"))
        .collect::<Vec<_>>();
    loop {
        if *ip > prog.len() - 2 { break; }
        let instr = prog[*ip];
        let mut op_val = prog[*ip + 1];
        let op = &mut op_val;
        INSTR_TBL[instr](op, a, b, c, ip, &mut out)
    }
    if let Some(ex_a) = ex_a { assert_eq!(*a, ex_a); }
    if let Some(ex_b) = ex_b { assert_eq!(*b, ex_b); }
    if let Some(ex_c) = ex_c { assert_eq!(*c, ex_c); }
    out
}
fn run_override_1str(d: &str, ov: usize) -> String {
    run_program_and_test(d, Some(ov), None, None, None)
}
fn run_override_2str(d: &str, ov: usize) -> (String, String) {
    (get_prog_str(d).to_string(), run_override_1str(d, ov))
}
fn find_override_linear(d: &str) -> usize {
    let prog_str = get_prog_str(d);
    for ov in 0.. {
        let res = run_override_1str(d, ov);
        if res == prog_str {
            return ov;
        }
    }
    unreachable!("find_override");
}
fn print_run_override_1str(d: &str, prog_str: &str, ov: usize) -> String {
    let s = Instant::now();
    let res = run_override_1str(d, ov);
    println!("{ov} {prog_str} {res} {:?}", s.elapsed());
    res
}
fn find_middle(d: &str, prog_str: &str, prog_str_len: usize) -> usize {
    let mut ov = 0;
    let mut s = 0;
    let mut e = usize::MAX;
    loop {
        let res = print_run_override_1str(d, prog_str, ov);
        if res == prog_str {
            return ov;
        }
        match res.len().cmp(&prog_str_len) {
            Ordering::Less => s = ov,
            Ordering::Equal => return ov,
            Ordering::Greater => e = ov,
        }
        ov = (e - s) / 2;
    }
}
fn find_start(d: &str, prog_str: &str, prog_str_len: usize, s: usize, e: usize, ss: usize) -> usize {
    #[allow(unused_assignments)]
    let mut s = s;
    let mut e = e;
    let mut ov = e;
    loop {
        let res = print_run_override_1str(d, prog_str, ov);
        if res == prog_str {
            return ov;
        }
        match res.len().cmp(&prog_str_len) {
            Ordering::Less => {
                if ss == 1 {
                    return ov + 1;
                }
                s = ov;
                break;
            },
            Ordering::Equal => {
                e = ov;
                ov -= ss;
            },
            Ordering::Greater => panic!("down gt"),
        }
    }
    find_start(d, prog_str, prog_str_len, s, e, ss / 2)
}
fn find_end(d: &str, prog_str: &str, prog_str_len: usize, s: usize, e: usize, ss: usize) -> usize {
    let mut s = s;
    #[allow(unused_assignments)]
    let mut e = e;
    let mut ov = s;
    loop {
        let res = print_run_override_1str(d, prog_str, ov);
        if res == prog_str {
            return ov;
        }
        match res.len().cmp(&prog_str_len) {
            Ordering::Greater => {
                if ss == 1 {
                    return ov - 1;
                }
                e = ov;
                break;
            },
            Ordering::Equal => {
                s = ov;
                ov += ss;
            },
            Ordering::Less => panic!("up lt"),
        }
    }
    find_end(d, prog_str, prog_str_len, s, e, ss / 2)
}
fn squeeze_bounds<T: Fn(usize) -> Ordering>(s: usize, e: usize, ss: usize, d: isize, check: &T) -> (usize, usize) {
    if d == 0 {
        let (s, e) = squeeze_bounds(s, e, ss, 1, check);
        return squeeze_bounds(s, e, ss, -1, check);
    }
    let mut s = s;
    let mut e = e;
    let mut ss = ss;
    let mut i = if d == 1 { s } else { e };
    loop {
        match check(i) {
            Ordering::Less => {
                if d == 1 {
                    s = i;
                } else {
                    ss /= 2;
                }
                i += ss;
            },
            Ordering::Equal => {
                if ss > 1 {
                    return squeeze_bounds(s, e, ss / 2, d, check);
                } else {
                    let se = if d == 1 { &mut s } else { &mut e };
                    *se = i;
                    return (s, e);
                }
            },
            Ordering::Greater => {
                if d == -1 {
                    e = i;
                } else {
                    ss /= 2;
                }
                i -= ss;
            },
        }
    }
}
fn find_override_alg(d: &str) -> usize {
    let prog_str = get_prog_str(d);
    let prog_str_len = prog_str.len();
    let (s, e) = squeeze_bounds(0, usize::MAX, usize::MAX / 1024, 0, &|i| {
        let res = print_run_override_1str(d, prog_str, i);
        if res == prog_str {
            panic!("FOUND IT {i}")
        }
        res.len().cmp(&prog_str_len)
    });
    println!("{s} {e} {} {}", s == 35184372088832, e == 281474976710655);

    0
}
fn main() {
    println!("START");
    println!("{}", find_override_alg(d()));
    // let s = Instant::now();
    // assert_eq!(117440, find_override(e7()));
    // println!("{:?}", s.elapsed());
}
#[cfg(test)]
mod test {
    use super::*;
    #[test] fn p1_e1() { assert_eq!("", run_program_and_test(e1(), None, None, Some(1), None)); }
    #[test] fn p1_e2() { assert_eq!("0,1,2", run_program_and_test(e2(), None, None, None, None)); }
    #[test] fn p1_e3() { assert_eq!("4,2,5,6,7,7,7,7,3,1,0", run_program_and_test(e3(), None, Some(0), None, None)); }
    #[test] fn p1_e4() { assert_eq!("", run_program_and_test(e4(), None, None, Some(26), None)); }
    #[test] fn p1_e5() { assert_eq!("", run_program_and_test(e5(), None, None, Some(44354), None)); }
    #[test] fn p1_e6() { assert_eq!("4,6,3,5,6,3,5,2,1,0", run_program_and_test(e6(), None, None, None, None)); }
    #[test] fn p1_d() { assert_eq!("6,5,4,7,1,6,0,3,1", run_program_and_test(d(), None, None, None, None)); }
    #[test] fn p2_e7() { let (a, b) = run_override_2str(e7(), 117440); assert_eq!(a, b); }
    #[test] fn p2_e7_2() { assert_eq!(117440, find_override_linear(e7())); }
}
