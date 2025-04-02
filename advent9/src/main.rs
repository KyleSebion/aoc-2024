#![allow(dead_code)]
use itertools::Itertools;
use std::{ops::Range, time::Instant};
fn get_p1_ex1() -> &'static str {
    "2333133121414131402"
}
fn get_p1_ex2() -> &'static str {
    "233313312141413140209"
}
fn get_data() -> &'static str {
    include_str!("input.txt")
}
fn disk_to_string(d: &[usize]) -> String {
    let mut s = String::new();
    for i in d {
        if *i == usize::MAX {
            s.push('.');
        } else {
            s.push_str(&i.to_string());
        }
    }
    s
}
const FREE_BLOCK: usize = usize::MAX;
fn mk_disk(d: &str) -> Vec<usize> {
    let mut v = Vec::with_capacity(d.len() * 9);
    let mut f = true;
    let mut id = 0;
    for c in d.chars() {
        let d = c.to_digit(10).expect("digit");
        for _ in 0..d {
            if f {
                v.push(id);
            } else {
                v.push(FREE_BLOCK);
            }
        }
        if f {
            id += 1;
        }
        f = !f;
    }
    v
}
fn defrag_disk_p1(d: &mut [usize]) {
    let mut last_rbi = d.len() - 1;
    for lbi in 0..d.len() {
        if lbi >= last_rbi {
            break;
        }
        if d[lbi] == FREE_BLOCK {
            let rbi_find_start = lbi + 1;
            let rbio = d[rbi_find_start..=last_rbi]
                .iter()
                .rposition(|&x| x != FREE_BLOCK);
            if let Some(rbi) = rbio {
                let rbi = rbi_find_start + rbi;
                d.swap(lbi, rbi);
                last_rbi = rbi - 1;
            } else {
                break;
            }
        }
    }
}
fn get_disk_checksum(d: &[usize]) -> usize {
    d.iter()
        .enumerate()
        .map(|(i, v)| if *v == FREE_BLOCK { 0 } else { i * *v })
        .sum()
}
fn analyze_disk(d: &[usize]) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
    let mut free = vec![];
    let mut used = vec![];
    for (k, v) in &d.iter().enumerate().chunk_by(|(_, &v)| v) {
        let r = match v.collect::<Vec<_>>()[..] {
            [] => panic!("empty not possible since chunk must have at least a size of 1"),
            [(s, _)] => s..s + 1,
            [(s, _), .., (e, _)] => s..e + 1,
        };
        if k == FREE_BLOCK {
            free.push(r);
        } else {
            used.push(r);
        }
    }
    (free, used)
}
fn defrag_disk_p2(d: &mut [usize]) -> Option<()> {
    let (mut free, mut used) = analyze_disk(d);
    loop {
        let u = used.pop()?;
        let rm_start = free.iter().rposition(|f| f.start < u.start)? + 1;
        if rm_start < free.len() {
            free.drain(rm_start..);
        }
        let u_len = u.len();
        if let Some(fi) = free.iter().position(|f| f.len() >= u_len) {
            let f = if free[fi].len() == u_len {
                free.remove(fi)
            } else {
                let f_s = free[fi].start;
                let f_e = free[fi].end;
                free[fi] = f_s + u_len..f_e;
                f_s..f_s + u_len
            };
            let (l, r) = d.split_at_mut(u.start);
            l[f].swap_with_slice(&mut r[0..u_len]);
        }
    }
}
fn main() {
    let r = 0..200;
    let r_len = r.len() as f64;
    let mut ms = Vec::with_capacity(r.len());
    for _ in r {
        let s = Instant::now();
        let mut d = mk_disk(get_data());
        defrag_disk_p2(&mut d);
        let _ = get_disk_checksum(&d);
        let d = s.elapsed();
        ms.push(d.as_secs_f64() * 1000_f64);
    }
    print!(
        "\
min ms: {:.1}
avg ms: {:.1}
max ms: {:.1}
tot ms: {:.0}
",
        ms.clone().into_iter().reduce(f64::min).unwrap(),
        ms.clone().into_iter().sum::<f64>() / r_len,
        ms.clone().into_iter().reduce(f64::max).unwrap(),
        ms.clone().into_iter().sum::<f64>(),
    );
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn p1_ex1_disk() {
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.888899",
            disk_to_string(&mk_disk(get_p1_ex1()))
        );
    }
    #[test]
    fn p1_ex1_defragged_disk() {
        let mut d = mk_disk(get_p1_ex1());
        defrag_disk_p1(&mut d);
        assert_eq!(
            "0099811188827773336446555566..............",
            disk_to_string(&d)
        );
    }
    #[test]
    fn p1_ex1_defragged_disk_checksum() {
        let mut d = mk_disk(get_p1_ex1());
        defrag_disk_p1(&mut d);
        assert_eq!(1928, get_disk_checksum(&d));
    }
    #[test]
    fn p1() {
        let mut d = mk_disk(get_data());
        defrag_disk_p1(&mut d);
        assert_eq!(6446899523367, get_disk_checksum(&d));
    }
    #[test]
    fn p2_test_free() {
        let d = mk_disk(get_p1_ex1());
        let (free, _) = analyze_disk(&d);
        assert_eq!(
            free,
            vec![2..5, 8..11, 12..15, 18..19, 21..22, 26..27, 31..32, 35..36,]
        );
    }
    #[test]
    fn p2_test_used() {
        let d = mk_disk(get_p1_ex1());
        let (_, used) = analyze_disk(&d);
        assert_eq!(
            used,
            vec![
                0..2,
                5..8,
                11..12,
                15..18,
                19..21,
                22..26,
                27..31,
                32..35,
                36..40,
                40..42,
            ]
        );
    }
    #[test]
    fn p2_ex1_defragged_disk() {
        let mut d = mk_disk(get_p1_ex1());
        defrag_disk_p2(&mut d);
        assert_eq!(
            "00992111777.44.333....5555.6666.....8888..",
            disk_to_string(&d)
        );
    }
    #[test]
    fn p2_ex1_defragged_disk_checksum() {
        let mut d = mk_disk(get_p1_ex1());
        defrag_disk_p2(&mut d);
        assert_eq!(2858, get_disk_checksum(&d));
    }
    #[test]
    fn p1_t2() {
        let mut d = mk_disk("12345");
        defrag_disk_p1(&mut d);
        assert_eq!(60, get_disk_checksum(&d));
    }
    #[test]
    fn p2_t2() {
        let mut d = mk_disk("12345");
        defrag_disk_p2(&mut d);
        assert_eq!(132, get_disk_checksum(&d));
    }
    #[test]
    fn p2_t3() {
        let mut d = mk_disk("80893804751608292");
        defrag_disk_p2(&mut d);
        assert_eq!(1715, get_disk_checksum(&d));
    }
    #[test]
    fn p2_t4() {
        let mut d = mk_disk("714892711");
        defrag_disk_p2(&mut d);
        assert_eq!(813, get_disk_checksum(&d));
    }
    #[test]
    fn p2_t5() {
        let mut d = mk_disk("12101");
        defrag_disk_p2(&mut d);
        assert_eq!(4, get_disk_checksum(&d));
    }
    #[test]
    fn p2_t6() {
        let mut d = mk_disk("233313312141413140211");
        defrag_disk_p2(&mut d);
        assert_eq!(2910, get_disk_checksum(&d));
    }
    #[test]
    fn p2_t7() {
        let mut d = mk_disk("1313165");
        defrag_disk_p2(&mut d);
        assert_eq!(169, get_disk_checksum(&d));
    }
    #[test]
    fn p2() {
        let mut d = mk_disk(get_data());
        defrag_disk_p2(&mut d);
        //assert_eq!(8666607636406, get_disk_checksum(&d));
        assert_eq!(6478232739671, get_disk_checksum(&d));
    }
}
