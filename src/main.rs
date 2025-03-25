#![allow(dead_code)]

use std::{collections::{HashMap, HashSet}, time::Instant};

use itertools::Itertools;

const fn d() -> &'static str {
    include_str!("input.txt")
}
const fn e1() -> &'static str {
    "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"
}

fn get_combos(d: &str) -> Vec<String> {
    d.lines().combinations(3).map(|v| v.join("-")).collect()
}
fn get_connected_3(d: &str) -> Vec<String> {
    get_combos(d)
        .into_iter()
        .map(|v| v.split("-").map(str::to_string).counts())
        .filter(|hm| hm.values().all(|k| k == &2))
        .map(|hm| hm.keys().join(","))
        .collect()
}
fn sort_connected_3(d: Vec<String>) -> Vec<String> {
    d.into_iter()
        .map(|v| v.split(",").sorted().join(","))
        .sorted()
        .collect()
}
fn filter_connected_3(d: Vec<String>) -> Vec<String> {
    d.into_iter()
        .filter(|v| v.split(",").any(|p| p.starts_with("t")))
        .collect()
}
fn get_combos_t_count(d: &str) -> usize {
    let mut map = HashMap::new();
    for l in d.lines() {
        let mut d = false;
        for (a, b) in l.split("-").tuple_windows() {
            if !d { d = true; } else { panic!("too many tuple_windows; expected 1; 2nd: ({a},{b})"); }
            map.entry(a).or_insert(HashSet::new()).insert(b);
            map.entry(b).or_insert(HashSet::new()).insert(a);
        }
    }
    map.retain(|k, v| k.starts_with("t") || v.iter().any(|v| v.starts_with("t")));
    map.keys()
        .cloned()
        .tuple_combinations()
        .filter(|&(a, b, c)| {
            (a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
                && map[a].contains(b)
                && map[a].contains(c)
                && map[b].contains(a)
                && map[b].contains(c)
                && map[c].contains(a)
                && map[c].contains(b)
        })
        .count()
}
fn main() {
    let s = Instant::now();
    println!("START");
    println!("{}", get_combos_t_count(d()));
    println!("END {:?}", s.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t_e1_3() {
        assert_eq!(
            vec![
                "aq,cg,yn", "aq,vc,wq", "co,de,ka", "co,de,ta", "co,ka,ta", "de,ka,ta", "kh,qp,ub",
                "qp,td,wh", "tb,vc,wq", "tc,td,wh", "td,wh,yn", "ub,vc,wq",
            ],
            sort_connected_3(get_connected_3(e1()))
        );
    }
    #[test]
    fn t_e1_3_t() {
        assert_eq!(
            vec![
                "co,de,ta", "co,ka,ta", "de,ka,ta", "qp,td,wh", "tb,vc,wq", "tc,td,wh", "td,wh,yn",
            ],
            sort_connected_3(filter_connected_3(get_connected_3(e1())))
        );
    }
    #[test]
    fn t_e1_3_count() {
        assert_eq!(7, get_combos_t_count(e1()));
    }
    #[test]
    fn t_d_3_count() {
        assert_eq!(1163, get_combos_t_count(d()));
    }
}
