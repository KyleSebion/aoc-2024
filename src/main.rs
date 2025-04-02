#![allow(dead_code)]

use std::collections::BTreeMap;

fn d() -> &'static str {
    include_str!("input.txt")
}
fn e1() -> &'static str {
    "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"
}

type LK = (char, u8, u8, u8, u8, u8);
type LKCont = BTreeMap<char, Vec<LK>>;
const KEY: char = 'K';
const LCK: char = 'L';
const EMP: char = '.';
const POP: char = '#';
const LCK_I: usize = 0;
const KEY_I: usize = 6;
const LK_WIDTH: usize = 5;
const LK_MAX: u8 = 5;
const LK_HEIGHT: usize = 7;
const LK_MARKER: &str = "#####";
fn parse_lock_or_key(d: &str) -> LK {
    let mut lk = ' ';
    let mut h = [0; LK_WIDTH];
    for (y, l) in d.lines().filter(|l| !l.is_empty()).enumerate() {
        if l == LK_MARKER {
            if y == LCK_I {
                lk = LCK;
                continue;
            } else if y == KEY_I {
                lk = KEY;
                continue;
            }
        }
        for (x, c) in l.char_indices() {
            if c == POP {
                h[x] += 1;
            }
        }
    }
    (lk, h[0], h[1], h[2], h[3], h[4])
}
fn parse_lks(d: &str) -> LKCont {
    let mut cont = BTreeMap::new();
    for i in d.split("\n\n") {
        let r = parse_lock_or_key(i);
        cont.entry(r.0).or_insert(Vec::new()).push(r);
    }
    cont
}
fn lk_heights(v: LK) -> Vec<u8> {
    vec![v.1, v.2, v.3, v.4, v.5]
}
fn lk_fit(a: LK, b: LK) -> bool {
    if (a.0 == KEY && b.0 == KEY) || (a.0 == LCK && b.0 == LCK) {
        panic!("mismatched {a:?} {b:?}");
    }
    lk_heights(a)
        .into_iter()
        .zip(lk_heights(b))
        .all(|(a, b)| a + b <= LK_MAX)
}
fn lk_count_fit(d: LKCont) -> usize {
    let mut count = 0;
    for &l in d[&LCK].iter() {
        for &k in d[&KEY].iter() {
            if lk_fit(l, k) {
                count += 1;
            }
        }
    }
    count
}
fn main() {
    println!("{}", lk_count_fit(parse_lks(d())));
}

#[cfg(test)] #[rustfmt::skip]
mod test {
    use super::*;
    #[test] fn t_parse_e1_lcks() { assert_eq!(vec![('L',0,5,3,4,3),('L',1,2,0,5,3)], parse_lks(e1())[&LCK]) }
    #[test] fn t_parse_e1_keys() { assert_eq!(vec![('K',5,0,2,1,3),('K',4,3,4,0,2),('K',3,0,2,0,1)], parse_lks(e1())[&KEY]) }
    #[test] fn t_parse_e1_fits() { assert_eq!(3, lk_count_fit(parse_lks(e1()))) }
    #[test] fn t_parse_d_fits() { assert_eq!(3495, lk_count_fit(parse_lks(d()))) }
}
