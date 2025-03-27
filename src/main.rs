#![allow(dead_code)]

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    time::Instant,
};

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
const fn e2() -> &'static str {
    "\
ka-co
ta-co
de-co
ta-ka
de-ta
ka-de
"
}
const fn e2_mod() -> &'static str {
    "\
ka-co
ta-co
de-co
ta-ka
de-ta
ka-de
ab-co
ac-co
ad-co
ac-ab
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
            if !d {
                d = true;
            } else {
                panic!("too many tuple_windows; expected 1; line: {l}");
            }
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
                && map[b].contains(c)
                && map[c].contains(a)
        })
        .count()
}
fn get_biggest_groups(d: &str) -> Vec<String> {
    let mut k1 = BTreeMap::new();
    for l in d.lines() {
        let mut d = false;
        for (a, b) in l.split("-").tuple_windows() {
            if !d {
                d = true;
            } else {
                panic!("too many tuple_windows; expected 1; line: {l}");
            }
            for v in [a, b].into_iter().permutations(2) {
                let a = v[0];
                let b = v[1];
                let e = k1
                    .entry(BTreeSet::from_iter([a]))
                    .or_insert(BTreeSet::new());
                e.insert(a);
                e.insert(b);
            }
        }
    }
    let max_possible_group_size = k1
        .values()
        .map(|v| v.len())
        .max()
        .expect("max_possible_group_size");
    let mut groups = vec![BTreeMap::new(); 1];
    let k1_i = groups.len();
    groups.push(k1);
    // groups[0] is unused
    for prev_i in groups.len() - 1..max_possible_group_size {
        let mut group = BTreeMap::new();
        let mut skip_list = BTreeSet::new();
        for ms_front in groups[prev_i].keys() {
            skip_list.extend(ms_front.iter().cloned());
            for ms_next in groups[prev_i][ms_front]
                .iter()
                .cloned()
                .filter(|&v| !skip_list.contains(v))
            {
                let ms_next_set = BTreeSet::from_iter([ms_next]);
                let fn_combined = ms_front
                    .union(&ms_next_set)
                    .cloned()
                    .collect::<BTreeSet<_>>();
                let fn_common = groups[prev_i][ms_front]
                    .intersection(&groups[k1_i][&ms_next_set])
                    .cloned()
                    .collect::<BTreeSet<_>>();
                let combined_in_common =
                    fn_combined.intersection(&fn_common).count() == fn_combined.len();
                if combined_in_common {
                    group.insert(fn_combined, fn_common);
                }
            }
        }
        if group.is_empty() {
            break;
        } else {
            groups.push(group);
        }
    }
    groups[groups.len() - 1]
        .keys()
        .map(|k| k.iter().join(","))
        .collect()
}
fn main() {
    let s = Instant::now();
    println!("START");
    println!("{:?}", get_biggest_groups(d())); // <80ms in release mode
    // println!("{}", get_combos_t_count(d()));
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
    #[test]
    fn t_d_best() {
        assert_eq!(
            vec!["bm,bo,ee,fo,gt,hv,jv,kd,md,mu,nm,wx,xh"],
            get_biggest_groups(d())
        );
    }
    #[test]
    fn t_e2_best() {
        assert_eq!(vec!["co,de,ka,ta"], get_biggest_groups(e2()));
    }
    #[test]
    fn t_e2_mod_best() {
        assert_eq!(vec!["co,de,ka,ta"], get_biggest_groups(e2_mod()));
    }
}
