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

const fn pow2(exp: usize) -> usize {
    1 << exp
}
fn adv_0(op: &mut usize, a: &mut usize, b: &mut usize, c: &mut usize, ip: &mut usize, _: &mut String) {
    *a /= pow2(*combo(op, a, b, c));
    *ip += 2;
}
fn bxl_1(op: &mut usize, _: &mut usize, b: &mut usize, _: &mut usize, ip: &mut usize, _: &mut String) {
    *b ^= *op;
    *ip += 2;
}
fn bst_2(op: &mut usize, a: &mut usize, b: &mut usize, c: &mut usize, ip: &mut usize, _: &mut String) {
    *b = *combo(op, a, b, c) % 8;
    *ip += 2;
}
fn jnz_3(op: &mut usize, a: &mut usize, _: &mut usize, _: &mut usize, ip: &mut usize, _: &mut String) {
    if *a == 0 {
        *ip += 2;
    } else {
        *ip = *op;
    }
}
fn bxc_4(_: &mut usize, _: &mut usize, b: &mut usize, c: &mut usize, ip: &mut usize, _: &mut String) {
    *b ^= *c;
    *ip += 2;
}
fn out_5(op: &mut usize, a: &mut usize, b: &mut usize, c: &mut usize, ip: &mut usize, out: &mut String) {
    let to_out = *combo(op, a, b, c) % 8;
    if !out.is_empty() {
        out.push(',');
    }
    out.push((to_out as u8 | 0x30) as char);
    *ip += 2;
}
fn bdv_6(op: &mut usize, a: &mut usize, b: &mut usize, c: &mut usize, ip: &mut usize, _: &mut String) {
    *b = *a / pow2(*combo(op, a, b, c));
    *ip += 2;
}
fn cdv_7(op: &mut usize, a: &mut usize, b: &mut usize, c: &mut usize, ip: &mut usize, _: &mut String) {
    *c = *a / pow2(*combo(op, a, b, c));
    *ip += 2;
}
fn do_instr(instr: usize, op: &mut usize, a: &mut usize, b: &mut usize, c: &mut usize, ip: &mut usize, out: &mut String) {
    match instr {
        INSTR_ADV => adv_0(op, a, b, c, ip, out),
        INSTR_BXL => bxl_1(op, a, b, c, ip, out),
        INSTR_BST => bst_2(op, a, b, c, ip, out),
        INSTR_JNZ => jnz_3(op, a, b, c, ip, out),
        INSTR_BXC => bxc_4(op, a, b, c, ip, out),
        INSTR_OUT => out_5(op, a, b, c, ip, out),
        INSTR_BDV => bdv_6(op, a, b, c, ip, out),
        INSTR_CDV => cdv_7(op, a, b, c, ip, out),
        _ => {},
    }
}

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
fn run_program(ov_a: Option<usize>, prog: &[usize]) -> String {
    let a = &mut 0;
    let b = &mut 0;
    let c = &mut 0;
    let ip = &mut 0;
    let mut out = String::with_capacity(32);
    if let Some(ov_a) = ov_a {
        *a = ov_a;
    }
    loop {
        if *ip > prog.len() - 2 { break; }
        let instr = prog[*ip];
        let mut op_val = prog[*ip + 1];
        let op = &mut op_val;
        do_instr(instr, op, a, b, c, ip, &mut out);
    }
    out
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
        // INSTR_TBL[instr](op, a, b, c, ip, &mut out);
        do_instr(instr, op, a, b, c, ip, &mut out);
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
fn squeeze_bounds<T: Fn(usize, isize) -> Ordering>(s: usize, e: usize, ss: usize, d: isize, check: &T) -> (usize, usize) {
    if d == 0 {
        let (s, e) = squeeze_bounds(s, e, ss, 1, check);
        return squeeze_bounds(s, e, ss, -1, check);
    }
    let mut s = s;
    let mut e = e;
    let mut ss = ss;
    let mut i = if d == 1 { s } else { e };
    loop {
        match check(i, d) {
            Ordering::Less => {
                if d == 1 { s = i; } else { ss /= 2; }
                i += ss;
            },
            Ordering::Equal => {
                if ss > 1 {
                    return squeeze_bounds(s, e, ss / 2, d, check);
                } else {
                    if d == 1 { s = i } else { e = i };
                    return (s, e);
                }
            },
            Ordering::Greater => {
                if d == -1 { e = i; } else { ss /= 2; }
                i -= ss;
            },
        }
    }
}
fn find_override_alg(d: &str) -> usize {
    //2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0
    let prog_str = get_prog_str(d);

    // let look_s = 8;
    // let look_l = 7;
    // let look_m = String::from_iter(prog_str.chars().skip(look_s).take(look_l));
    // println!("{look_m}"); // 7,5,1,6,0,3
    // let (s, e) = squeeze_bounds(s, e, (e - s) / 102400, 0, &|i, dir| {
    //     let res = print_run_override_1str(d, prog_str, i);
    //     let res_m = String::from_iter(res.chars().skip(look_s).take(look_l));
    //     if res_m == look_m {
    //         return Ordering::Equal;
    //     }
    //     if i > e { return Ordering::Greater; }
    //     if i < s { return Ordering::Less; }
    //     if dir == 1 {
    //         Ordering::Less
    //     } else if dir == -1 {
    //         Ordering::Greater
    //     } else {
    //         panic!("no")
    //     }
    // });
    // println!("{s} {e}");

    //prog: 2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0
    //last 6 (excluding very last) seem to change at the same time
    //1. 35184372088832..=281474976710655 based on length
    //  a1. let prog_str_len = prog_str.len(); let (s, e) = squeeze_bounds(0, usize::MAX, usize::MAX / 1024, 0, &|i, _| { print_run_override_1str(d, prog_str, i).len().cmp(&prog_str_len) });
    //  a2. println!("{s} {e} {} {}", s == 35184372088832, e == 281474976710655);
    //  b1. had what seemed to be good luck with:
    //      for i in (s+11643290..e).step_by(16777216 * 64) {
    //          for a in [0, 4194304, 4227072] {
    //              if r.starts_with("2,4,1,5,7,5,1") && r != last && r.ends_with("5,5,3,0") {
    //  b2. 16777216 based on pinkish purple at https://docs.google.com/spreadsheets/d/17VCj4QsdiONDWZhHMegegn1b1qx7dFN0mdwcOXjp4mw/edit?gid=1710713870#gid=1710713870
    //2. 108995544197530..=136956859263386 based on checking first 7 and last 6 in steps of 16777216
    //3. below 109558863325594 because advent said too high
    //4. (this could be wrong because i checked sub-ranges of them) below 109146941794714 (nothing good from it to 109558863325594)
    #[allow(clippy::single_element_loop)]
    for range in [108995544197530..=109146941794714] { //none
        let prog = &prog_str
            .split(',')
            .map(|v| v.parse::<usize>().expect("instr"))
            .collect::<Vec<_>>();
        let thread_cnt = 12;
        let start = range.start();
        let size = (range.end() - start) / thread_cnt;
        let tids = Vec::from_iter(0..thread_cnt);
        std::thread::scope(|s|{
            for i in tids.iter() {
                s.spawn(|| {
                    let prog = &prog.clone()[..];
                    let i = *i;
                    let s = start + i * size;
                    let e = s + size;
                    for ov in s..e {
                        let r = run_program(Some(ov), prog);
                        if r == "2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0" {
                            println!("{ov} {prog_str} {r}");
                        }
                    }
                });
            }
        });
    }
    0
}
fn tmp_1(d: &str) {
    // cuda: ~224,757,102 per sec (7x faster)
    // mt:    ~32,343,872 per sec
    let s = Instant::now();
    //2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0
    let prog_str = get_prog_str(d);
    const CUDA_SIZE: usize = 1024*1024*222;
    const LOOP_END: usize = 16;
    #[allow(clippy::single_element_loop, clippy::single_range_in_vec_init)]
    for range in [0..CUDA_SIZE*LOOP_END] {
        let prog = &prog_str
            .split(',')
            .map(|v| v.parse::<usize>().expect("instr"))
            .collect::<Vec<_>>();
        let thread_cnt = 12;
        let start = range.start;
        let size = (range.end - start) / thread_cnt;
        let tids = Vec::from_iter(0..thread_cnt);
        std::thread::scope(|s|{
            for i in tids.iter() {
                s.spawn(|| {
                    let prog = &prog.clone()[..];
                    let i = *i;
                    let s = start + i * size;
                    let e = if i == thread_cnt - 1 { range.end } else { s + size };
                    for ov in s..e {
                        let r = run_program(Some(ov), prog);
                        if ov % CUDA_SIZE == 0 || ov == CUDA_SIZE * LOOP_END - 1 {
                            println!("{ov} {r}; ");
                        }
                    }
                });
            }
        });
    }
    println!("END {:?}", s.elapsed());
}
fn main() {
    println!("START");
    tmp_1(d());
    // println!("{}", find_override_alg(d()));
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
