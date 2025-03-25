#![allow(dead_code, clippy::let_and_return)]

use std::collections::HashMap;
use itertools::Itertools;

const fn d() -> &'static str {
    include_str!("input.txt")
}
const fn e1() -> &'static str {
    "\
1
10
100
2024
"
}
const fn e2() -> &'static str {
    "\
1
2
3
2024
"
}

fn get_next_secret_verbose(s: usize) -> usize {
    let s1 = s * 64;
    let s = s ^ s1;
    let s = s % 16777216;
    let s2 = s / 32;
    let s = s ^ s2;
    let s = s % 16777216;
    let s3 = s * 2048;
    let s = s ^ s3;
    let s = s % 16777216;
    s
}
fn get_next_secret_terse(s0: usize) -> usize {
    let s1 = (16777216 - 1) & (s0 ^ (s0 << 6));
    let s2 = (16777216 - 1) & (s1 ^ (s1 >> 5));
    let s3 = (16777216 - 1) & (s2 ^ (s2 << 11));
    s3
}
fn get_next_secret(s: usize) -> usize {
    get_next_secret_terse(s)
}
fn get_next_secret_nth(s: usize, nth: usize) -> usize {
    (0..nth).fold(s, |s, _| get_next_secret(s))
}
fn get_next_secrets_nth(d: &str, nth: usize) -> Vec<usize> {
    d.lines()
        .map(|s| get_next_secret_nth(s.parse::<usize>().unwrap(), nth))
        .collect()
}
fn get_next_secrets_nth_sum(d: &str, nth: usize) -> usize {
    get_next_secrets_nth(d, nth).into_iter().sum()
}
fn get_secret_diffs(s: usize, nth: usize) -> HashMap<(i8, i8, i8, i8), usize> {
    let mut last = s;
    (0..nth)
        .map(|_| {
            let res = get_next_secret(last);
            let l10 = last % 10;
            let r10 = res % 10;
            let diff = r10 as i8 - l10 as i8;
            last = res;
            (diff, r10)
        })
        // .inspect(|v| println!("{} ({})", v.1, v.0))
        .tuple_windows::<(_, _, _, _)>()
        .fold(HashMap::new(), |mut a, w| {
            a.entry((w.0.0, w.1.0, w.2.0, w.3.0)).or_insert(w.3.1);
            a
        })
}
fn print_123_secret_diffs() {
    for (k, v) in get_secret_diffs(123, 9) {
        println!("{k:?} {v}");
    }
}
fn get_secrets_diffs(d: &str, nth: usize) -> Vec<HashMap<(i8, i8, i8, i8), usize>> {
    d.lines()
        .map(|s| get_secret_diffs(s.parse::<usize>().unwrap(), nth))
        .collect()
}
fn get_best_secrets_diffs(d: &str, nth: usize) -> usize {
    let diffs = get_secrets_diffs(d, nth);
    let mut sums = HashMap::new();
    for hm in diffs {
        for (k, v) in hm {
            *sums.entry(k).or_insert(0) += v;
        }
    }
    sums.into_iter().max_by_key(|v| v.1).expect("get_best_secrets_diffs").1
}
fn main() {
    println!("{:?}", get_best_secrets_diffs(d(), 2000));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t_123() {
        let mut start = 123;
        let rs = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        for next in rs {
            let r1 = get_next_secret_terse(start);
            let r2 = get_next_secret_verbose(start);
            assert_eq!(r1, r2);
            assert_eq!(r1, next);
            start = next;
        }
    }
    #[test]
    fn t_123_nth10() {
        assert_eq!(5908254, get_next_secret_nth(123, 10))
    }
    #[test]
    fn t_e1_each() {
        assert_eq!(
            vec![8685429, 4700978, 15273692, 8667524],
            get_next_secrets_nth(e1(), 2000)
        )
    }
    #[test]
    fn t_e1_sum() {
        assert_eq!(37327623, get_next_secrets_nth_sum(e1(), 2000))
    }
    #[test]
    fn t_d_sum() {
        assert_eq!(19927218456, get_next_secrets_nth_sum(d(), 2000))
    }
    #[test]
    fn t_e2_best() {
        assert_eq!(23, get_best_secrets_diffs(e2(), 2000));
    }
    #[test]
    fn t_d_best() {
        assert_eq!(2189, get_best_secrets_diffs(d(), 2000));
    }
}
