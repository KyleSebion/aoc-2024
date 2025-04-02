#![allow(dead_code, unused_imports)]

const fn e1() -> &'static str {
    "\
029A
980A
179A
456A
379A
"
}
const fn d() -> &'static str {
    include_str!("input.txt")
}

use itertools::Itertools;
use std::{collections::{BTreeMap, HashMap}, iter::once, time::Instant};
fn merge_combos(mut vs: Vec<Vec<String>>) -> Vec<String> {
    let mut nv = vec!["".to_owned()];
    while let Some(vp) = vs.pop() {
        let mut nnv = Vec::new();
        for ns in vp {
            for os in &nv {
                nnv.push(format!("{ns}{os}"));
            }
        }
        nv = nnv;
    }
    nv
}
fn code_to_num(code: &str) -> usize {
    code
        .trim_start_matches('0')
        .trim_end_matches('A')
        .parse()
        .expect("parse_to_num failed")
}
fn get_dir_pad_moves() -> BTreeMap<(char, char), Vec<String>> {
    // i verified this is correct in the other program
    [
        (('A', 'A'), vec![""]),
        (('<', '<'), vec![""]),
        (('>', '>'), vec![""]),
        (('^', '^'), vec![""]),
        (('v', 'v'), vec![""]),
        (('A', '<'), vec!["v<<", "<v<"]), // adds 2 extra chars; removed 
        // (('A', '<'), vec!["v<<"]), // adds 2 extra chars; removed <v<
        (('A', '>'), vec!["v"]),
        (('A', '^'), vec!["<"]),
        (('A', 'v'), vec!["<v", "v<"]), // similar, but 1 adds a permutation; removed 
        // (('A', 'v'), vec!["<v"]), // similar, but 1 adds a permutation; removed v<
        (('<', 'A'), vec![">>^", ">^>"]), // adds 2 extra chars; removed 
        // (('<', 'A'), vec![">>^"]), // adds 2 extra chars; removed >^>
        (('<', '>'), vec![">>"]),
        (('<', '^'), vec![">^"]),
        (('<', 'v'), vec![">"]),
        (('>', 'A'), vec!["^"]),
        (('>', '<'), vec!["<<"]),
        (('>', '^'), vec!["<^", "^<"]), // i think i only need 1 and removing the other doesn't matter; removed 
        // (('>', '^'), vec!["<^"]), // i think i only need 1 and removing the other doesn't matter; removed ^<
        (('>', 'v'), vec!["<"]),
        (('^', 'A'), vec![">"]),
        (('^', '<'), vec!["v<"]),
        (('^', '>'), vec![">v", "v>"]), // i think i only need 1 and removing the other doesn't matter; removed 
        // (('^', '>'), vec![">v"]), // i think i only need 1 and removing the other doesn't matter; removed v>
        (('^', 'v'), vec!["v"]),
        (('v', 'A'), vec![">^", "^>"]), // i think i only need 1 and removing the other doesn't matter; removed 
        // (('v', 'A'), vec![">^"]), // i think i only need 1 and removing the other doesn't matter; removed ^>
        (('v', '<'), vec!["<"]),
        (('v', '>'), vec![">"]),
        (('v', '^'), vec!["^"]),
    ]
    .into_iter()
    .map(|(k, mut v)| {
        v.sort();
        (k, v.into_iter().map(|v| v.to_string() + "A").collect())
    })
    .collect()
}
fn get_num_pad_moves() -> BTreeMap<(char, char), Vec<String>> {
    // i verified this is correct in the other program
    [
        (('0', '0'), vec![""]),
        (('1', '1'), vec![""]),
        (('2', '2'), vec![""]),
        (('3', '3'), vec![""]),
        (('4', '4'), vec![""]),
        (('5', '5'), vec![""]),
        (('6', '6'), vec![""]),
        (('7', '7'), vec![""]),
        (('8', '8'), vec![""]),
        (('9', '9'), vec![""]),
        (('A', 'A'), vec![""]),
        (('0', '1'), vec!["^<"]),
        (('0', '2'), vec!["^"]),
        (('0', '3'), vec![">^", "^>"]),
        (('0', '4'), vec!["^<^", "^^<"]),
        (('0', '5'), vec!["^^"]),
        (('0', '6'), vec![">^^", "^>^", "^^>"]),
        (('0', '7'), vec!["^<^^", "^^<^", "^^^<"]),
        (('0', '8'), vec!["^^^"]),
        (('0', '9'), vec![">^^^", "^>^^", "^^>^", "^^^>"]),
        (('0', 'A'), vec![">"]),
        (('1', '0'), vec![">v"]),
        (('1', '2'), vec![">"]),
        (('1', '3'), vec![">>"]),
        (('1', '4'), vec!["^"]),
        (('1', '5'), vec![">^", "^>"]),
        (('1', '6'), vec![">>^", ">^>", "^>>"]),
        (('1', '7'), vec!["^^"]),
        (('1', '8'), vec![">^^", "^>^", "^^>"]),
        (('1', '9'), vec![">>^^", ">^>^", ">^^>", "^>>^", "^>^>", "^^>>"]),
        (('1', 'A'), vec![">>v", ">v>"]),
        (('2', '0'), vec!["v"]),
        (('2', '1'), vec!["<"]),
        (('2', '3'), vec![">"]),
        (('2', '4'), vec!["<^", "^<"]),
        (('2', '5'), vec!["^"]),
        (('2', '6'), vec![">^", "^>"]),
        (('2', '7'), vec!["<^^", "^<^", "^^<"]),
        (('2', '8'), vec!["^^"]),
        (('2', '9'), vec![">^^", "^^>", "^>^"]), // adds 2 chars; removed 
        // (('2', '9'), vec![">^^", "^^>"]), // adds 2 chars; removed ^>^
        (('2', 'A'), vec![">v", "v>"]),
        (('3', '0'), vec!["<v", "v<"]),
        (('3', '1'), vec!["<<"]),
        (('3', '2'), vec!["<"]),
        (('3', '4'), vec!["<<^", "<^<", "^<<"]),
        (('3', '5'), vec!["<^", "^<"]),
        (('3', '6'), vec!["^"]),
        (('3', '7'), vec!["<<^^", "<^<^", "<^^<", "^<<^", "^<^<", "^^<<"]),
        (('3', '8'), vec!["<^^", "^<^", "^^<"]),
        (('3', '9'), vec!["^^"]),
        (('3', 'A'), vec!["v"]),
        (('4', '0'), vec![">vv", "v>v"]),
        (('4', '1'), vec!["v"]),
        (('4', '2'), vec![">v", "v>"]),
        (('4', '3'), vec![">>v", ">v>", "v>>"]),
        (('4', '5'), vec![">"]),
        (('4', '6'), vec![">>"]),
        (('4', '7'), vec!["^"]),
        (('4', '8'), vec![">^", "^>"]),
        (('4', '9'), vec![">>^", ">^>", "^>>"]),
        (('4', 'A'), vec![">>vv", ">v>v", ">vv>", "v>>v", "v>v>"]),
        (('5', '0'), vec!["vv"]),
        (('5', '1'), vec!["<v", "v<"]),
        (('5', '2'), vec!["v"]),
        (('5', '3'), vec![">v", "v>"]),
        (('5', '4'), vec!["<"]),
        (('5', '6'), vec![">"]),
        (('5', '7'), vec!["<^", "^<"]),
        (('5', '8'), vec!["^"]),
        (('5', '9'), vec![">^", "^>"]),
        (('5', 'A'), vec![">vv", "v>v", "vv>"]),
        (('6', '0'), vec!["<vv", "v<v", "vv<"]),
        (('6', '1'), vec!["<<v", "<v<", "v<<"]),
        (('6', '2'), vec!["<v", "v<"]),
        (('6', '3'), vec!["v"]),
        (('6', '4'), vec!["<<"]),
        (('6', '5'), vec!["<"]),
        (('6', '7'), vec!["<<^", "<^<", "^<<"]),
        (('6', '8'), vec!["<^", "^<"]),
        (('6', '9'), vec!["^"]),
        (('6', 'A'), vec!["vv"]),
        (('7', '0'), vec![">vvv", "v>vv", "vv>v"]),
        (('7', '1'), vec!["vv"]),
        (('7', '2'), vec![">vv", "v>v", "vv>"]),
        (('7', '3'), vec![">>vv", ">v>v", ">vv>", "v>>v", "v>v>", "vv>>"]),
        (('7', '4'), vec!["v"]),
        (('7', '5'), vec![">v", "v>"]),
        (('7', '6'), vec![">>v", ">v>", "v>>"]),
        (('7', '8'), vec![">"]),
        (('7', '9'), vec![">>"]),
        (('7', 'A'), vec![">>vvv", ">v>vv", ">vv>v", ">vvv>", "v>>vv", "v>v>v", "v>vv>", "vv>>v", "vv>v>"]),
        (('8', '0'), vec!["vvv"]),
        (('8', '1'), vec!["<vv", "v<v", "vv<"]),
        (('8', '2'), vec!["vv"]),
        (('8', '3'), vec![">vv", "v>v", "vv>"]),
        (('8', '4'), vec!["<v", "v<"]),
        (('8', '5'), vec!["v"]),
        (('8', '6'), vec![">v", "v>"]),
        (('8', '7'), vec!["<"]),
        (('8', '9'), vec![">"]),
        (('8', 'A'), vec![">vvv", "v>vv", "vv>v", "vvv>"]),
        (('9', '0'), vec!["<vvv", "v<vv", "vv<v", "vvv<"]),
        (('9', '1'), vec!["<<vv", "<v<v", "<vv<", "v<<v", "v<v<", "vv<<"]),
        (('9', '2'), vec!["<vv", "v<v", "vv<"]),
        (('9', '3'), vec!["vv"]),
        (('9', '4'), vec!["<<v", "<v<", "v<<"]),
        (('9', '5'), vec!["<v", "v<"]),
        (('9', '6'), vec!["v"]),
        (('9', '7'), vec!["<<"]),
        (('9', '8'), vec!["<"]),
        (('9', 'A'), vec!["vvv"]),
        (('A', '0'), vec!["<"]),
        (('A', '1'), vec!["<^<", "^<<"]),
        (('A', '2'), vec!["<^", "^<"]),
        (('A', '3'), vec!["^"]),
        (('A', '4'), vec!["<^<^", "<^^<", "^<<^", "^<^<", "^^<<"]),
        (('A', '5'), vec!["<^^", "^<^", "^^<"]),
        (('A', '6'), vec!["^^"]),
        (('A', '7'), vec!["<^<^^", "<^^<^", "<^^^<", "^<<^^", "^<^<^", "^<^^<", "^^<<^", "^^<^<", "^^^<<"]), // adds 4 extra chars; removed 

/*
<^<^^ v<<A>^Av<A>^AA>A
<^^<^ v<<A>^AAv<A>^A>A
<^^^< v<<A>^AAAv<A>>^A
^<^<^ <Av<A>^Av<A>^A>A
^<^^< <Av<A>^AAv<A>>^A
^^<^< <AAv<A>^Av<A>>^A
^<<^^ <Av<AA>^AA>A
^^<<^ <AAv<AA>^A>A
^^^<< <AAAv<AA>>^A
*/

        (('A', '8'), vec!["<^^^", "^<^^", "^^<^", "^^^<"]),
        (('A', '9'), vec!["^^^"]),
    ]
    .into_iter()
    .map(|(k, mut v)| {
        v.sort();
        (k, v.into_iter().map(|v| v.to_string() + "A").collect())
    })
    .collect()
}
fn expand_to_dirs(vals: Vec<String>, moves: BTreeMap<(char, char), Vec<String>>) -> Vec<String> {
    vals.into_iter().flat_map(|s| {
        let v = once('A')
            .chain(s.chars())
            .tuple_windows::<(_, _)>()
            .map(|se| moves[&se].clone().into_iter().sorted().collect_vec())
            .collect_vec();
        // println!("{v:?}");
        merge_combos(v)
    }).collect()
}
fn p_slice(s: &[String]) {
    for i in s {
        println!("{i}");
    }
}
fn get_code_complexity(code: &str, n: usize) -> usize {
    let nd = expand_to_dirs(vec![code.to_string()], get_num_pad_moves());
    // p_slice(&nd);
    let mut dd = nd;
    for _ in 0..n {
        dd = expand_to_dirs(dd, get_dir_pad_moves());
    }
    // p_slice(&dd);
    let mut res = usize::MAX;
    for i in dd {
        if i.len() < res {
            res = i.len();
        }
    }
    res * code_to_num(code)
}
fn get_code_complexity2_dir(code: &str, n: usize, cache: &mut HashMap<(String, usize), usize>) -> usize {
    let cache_key = (code.to_string(), n);
    if cache.contains_key(&cache_key) {
        cache[&cache_key]
    } else if n == 0 {
        cache.insert(cache_key.clone(), code.len());
        cache[&cache_key]
    } else {
        let map = get_dir_pad_moves();
        let mut lr_best = 0;
        for lr in once('A').chain(code.chars()).tuple_windows::<(_, _)>() {
            let mut exp_best = usize::MAX;
            for exp in &map[&lr] {
                let exp_len = get_code_complexity2_dir(exp, n - 1, cache);
                if exp_len < exp_best {
                    exp_best = exp_len;
                }
            }
            lr_best += exp_best;
        }
        cache.insert(cache_key.clone(), lr_best);
        cache[&cache_key]
    }
}
fn get_code_complexity2(code: &str, n: usize) -> usize {
    let mut cache = HashMap::new();
    let map = get_num_pad_moves();
    let mut lr_best = 0;
    for lr in once('A').chain(code.chars()).tuple_windows::<(_, _)>() {
        let mut exp_best = usize::MAX;
        for exp in &map[&lr] {
            let exp_len = get_code_complexity2_dir(exp, n - 1, &mut cache);
            if exp_len < exp_best {
                exp_best = exp_len;
            }
        }
        lr_best += exp_best;
    }
    lr_best * code_to_num(code)
}
fn get_codes_complexity2(codes: &str, n: usize) -> usize {
    codes.lines().map(|code| get_code_complexity2(code, n)).sum()
}
fn main() {
    println!("START");
    // for (k, v) in get_dir_pad_moves() {
    //     println!("{k:?} {v:?}");
    // }
    // let pm = get_dir_pad_moves();
    // for (k, v) in get_dir_pad_moves() {
    //     if k.0 != 'A' || k.1 != 'v' || k.1 != '#' {
    //         continue;
    //     }
    //     for vi in v.iter() {
    //         let vi = "A".to_string() + vi;
    //         let vv = vi
    //             .chars()
    //             .tuple_windows::<(_, _)>()
    //             .map(|se| pm[&se].clone().into_iter().sorted().collect_vec())
    //             .collect_vec();
    //         const WIDTH: usize = 9;
    //         print!("{k:?} {v:?}{vi:^WIDTH$}");
    //         for vvi in vv {
    //             print!("{:^WIDTH$}", vvi.join("|"));
    //         }
    //         println!();
    //     }
    // }
    // println!("END1");
    // for (k, v) in get_num_pad_moves() {
    //     println!("{k:?} {v:?}");
    // }
    // let pm = get_dir_pad_moves();
    // for (k, v) in get_num_pad_moves() {
    //     if k.0 != 'A' || k.1 != '7' {
    //         continue;
    //     }
    //     for vi in v.iter() {
    //         let vi = "A".to_string() + vi;
    //         let vv = vi
    //             .chars()
    //             .tuple_windows::<(_, _)>()
    //             .map(|se| pm[&se].clone().into_iter().sorted().collect_vec())
    //             .collect_vec();
    //         const WIDTH: usize = 9;
    //         print!("{k:?} {v:?}{vi:^WIDTH$}");
    //         for vvi in vv {
    //             // print!("{:^WIDTH$}", vvi.join("|"));
    //             print!("{}", vvi.join(""));
    //         }
    //         println!();
    //     }
    // }
    // println!("END2");

    assert_eq!(68 * 29, get_code_complexity("029A", 2));
    assert_eq!(68 * 29, get_code_complexity2("029A", 3));

    // let depth = 25;
    // let s = Instant::now();
    // println!("{} {:?}", get_code_complexity2("029A", depth + 1), s.elapsed());
    // let s = Instant::now();
    // println!("{} {:?}", get_code_complexity("029A", depth), s.elapsed());

    let s = Instant::now();
    println!("{} {:?}", get_codes_complexity2(d(), 26), s.elapsed()); // less than 20 ms
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_code_complexity_1() {
        assert_eq!(68 * 29, get_code_complexity2("029A", 3));
    }
    #[test]
    fn get_code_complexity_2() {
        assert_eq!(60 * 980, get_code_complexity2("980A", 3));
    }
    #[test]
    fn get_code_complexity_3() {
        assert_eq!(68 * 179, get_code_complexity2("179A", 3));
    }
    #[test]
    fn get_code_complexity_4() {
        assert_eq!(64 * 456, get_code_complexity2("456A", 3));
    }
    #[test]
    fn get_code_complexity_5() {
        assert_eq!(64 * 379, get_code_complexity2("379A", 3));
    }
    #[test]
    fn get_codes_complexity2_e1() {
        assert_eq!(68 * 29 + 60 * 980 + 68 * 179 + 64 * 456 + 64 * 379, 126384);
        assert_eq!(
            68 * 29 + 60 * 980 + 68 * 179 + 64 * 456 + 64 * 379,
            get_codes_complexity2(e1(), 3)
        );
    }
    #[test]
    fn get_codes_complexity2_d() {
        assert_eq!(174124, get_codes_complexity2(d(), 3));
    }
    #[test]
    fn get_codes_complexity2_d_p2() {
        assert_eq!(216668579770346, get_codes_complexity2(d(), 26));
    }
}
