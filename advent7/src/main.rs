use std::time::Instant;
#[allow(dead_code)]
fn get_ex() -> &'static str {
    "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
}
fn get_data() -> &'static str {
    include_str!("input.txt")
}
#[allow(dead_code)]
fn get_tot_calib_res_p1(d: &str) -> usize {
    let mut total_res = 0;
    for l in d.lines() {
        let mut f = true;
        let v = l.split(" ").map(move |s| {
            let r = if f { ..s.len()-1 } else { ..s.len() };
            f = false;
            s[r].parse::<usize>().unwrap()
        }).collect::<Vec<_>>();
        let [expected_res, operands @ ..] = &v[..] else { panic!("{l} unexpected!") };
        for bits in 0_usize .. 1 << (operands.len()-1) {
            let mut calc_res = operands[0];
            for (i, operand) in operands[1..].iter().enumerate() {
                if bits >> i & 1 == 0 {
                    calc_res += operand;
                } else {
                    calc_res *= operand;
                }
            }
            if &calc_res == expected_res {
                total_res += calc_res;
                break;
            }
        }
    }
    total_res
}
fn inc_trits(trits: &mut [usize]) -> bool {
    for trit in trits {
        if *trit == 2 {
            *trit = 0;
        } else {
            *trit += 1;
            return true;
        }
    }
    false
}
fn get_tot_calib_res_p2(d: &str) -> usize {
    let mut total_res = 0;
    for l in d.lines() {
        let mut f = true;
        let v = l.split(" ").map(move |s| {
            let r = if f { ..s.len()-1 } else { ..s.len() };
            f = false;
            s[r].parse::<usize>().unwrap()
        }).collect::<Vec<_>>();
        let [expected_res, operands @ ..] = &v[..] else { panic!("{l} unexpected!") };
        let trits = &mut vec![0; operands.len() - 1];
        loop {
            let mut calc_res = operands[0];
            for (i, operand) in operands[1..].iter().enumerate() {
                match trits[i] {
                    0 => calc_res += operand,
                    1 => calc_res *= operand,
                    2 => calc_res = calc_res * 10_usize.pow(operand.ilog10() + 1) + operand,
                    _ => panic!("unexpected trit"),
                }
            }
            if &calc_res == expected_res {
                total_res += calc_res;
                break;
            }
            if !inc_trits(trits) { break; }
        }
    }
    total_res
}
#[allow(dead_code)]
fn super_slow_get_tot_calib_res_p2_radix_fmt(d: &str) -> usize {
    let mut total_res = 0;
    for l in d.lines() {
        let mut f = true;
        let v = l.split(" ").map(move |s| {
            let r = if f { ..s.len()-1 } else { ..s.len() };
            f = false;
            s[r].parse::<usize>().unwrap()
        }).collect::<Vec<_>>();
        let [expected_res, operands @ ..] = &v[..] else { panic!("{l} unexpected!") };
        let trit_len = operands.len() - 1;
        let trit_s = 32 - trit_len;
        for trits in 0..3_usize.pow(trit_len as u32) {
            let trits = format!("{:0>32}", radix_fmt::radix_3(trits).to_string())[trit_s..].chars().collect::<Vec<_>>();
            let mut calc_res = operands[0];
            for (i, operand) in operands[1..].iter().enumerate() {
                match trits[i] {
                    '0' => calc_res += operand,
                    '1' => calc_res *= operand,
                    '2' => calc_res = calc_res * 10_usize.pow(operand.ilog10() + 1) + operand,
                    _ => panic!("unexpected trit"),
                }
            }
            if &calc_res == expected_res {
                total_res += calc_res;
                break;
            }
        }
    }
    total_res
}
fn main() {
    let s = Instant::now();
    println!("{} {:?}", get_tot_calib_res_p2(get_data()), s.elapsed());
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test] fn p1_ex() { assert_eq!(3749, get_tot_calib_res_p1(get_ex())); }
    #[test] fn p1() { assert_eq!(4364915411363, get_tot_calib_res_p1(get_data())); }
    #[test] fn p2_ex() { assert_eq!(11387, get_tot_calib_res_p2(get_ex())); }
    #[test] fn p2_ex_radix_fmt() { assert_eq!(11387, super_slow_get_tot_calib_res_p2_radix_fmt(get_ex())); }
    #[test] fn p2() { assert_eq!(38322057216320, get_tot_calib_res_p2(get_data())); }
    #[test] fn test_trits() {
        let trits = &mut vec![0; 2];
        assert_eq!(trits, &mut vec![0, 0]); assert!(inc_trits(trits));
        assert_eq!(trits, &mut vec![1, 0]); assert!(inc_trits(trits));
        assert_eq!(trits, &mut vec![2, 0]); assert!(inc_trits(trits));
        assert_eq!(trits, &mut vec![0, 1]); assert!(inc_trits(trits));
        assert_eq!(trits, &mut vec![1, 1]); assert!(inc_trits(trits));
        assert_eq!(trits, &mut vec![2, 1]); assert!(inc_trits(trits));
        assert_eq!(trits, &mut vec![0, 2]); assert!(inc_trits(trits));
        assert_eq!(trits, &mut vec![1, 2]); assert!(inc_trits(trits));
        assert_eq!(trits, &mut vec![2, 2]); assert!(!inc_trits(trits));
        assert_eq!(trits, &mut vec![0, 0]); 
    }
}