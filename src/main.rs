#![allow(dead_code)]

use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
    usize,
};

const fn d() -> &'static str {
    include_str!("input.txt")
}
const fn e1() -> &'static str {
    "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"
}
const fn e2() -> &'static str {
    "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"
}

#[derive(Debug, Clone)]
enum Op {
    Wire,
    And,
    Or,
    Xor,
}
impl Op {
    fn new(d: &str) -> Self {
        match d {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            "" => Op::Wire,
            _ => {
                panic!("invalid op");
            }
        }
    }
    fn exec(&self, ia: Option<u8>, ib: Option<u8>) -> Option<u8> {
        match self {
            Self::Wire => ia,
            Self::And => Some(ia? & ib?),
            Self::Or => Some(ia? | ib?),
            Self::Xor => Some(ia? ^ ib?),
        }
    }
    fn is_wire(&self) -> bool {
        matches!(self, Self::Wire)
    }
}
#[derive(Debug, Clone)]
struct Gate<'a> {
    op: Op,
    iw: [&'a str; 2],
    iv: [Option<u8>; 2],
    ow: &'a str,
    ov: Option<u8>,
}
impl<'a> Gate<'a> {
    fn new(d: &'a str) -> Self {
        let d = d.split(": ").collect_vec();
        if d.len() == 2 {
            Self {
                op: Op::new(""),
                iw: [d[0]; 2],
                iv: [Some(d[1].parse().expect("ov parse failed")); 2],
                ow: d[0],
                ov: None,
            }
        } else {
            let d = d[0].split(" ").collect_vec();
            Self {
                op: Op::new(d[1]),
                iw: [d[0], d[2]],
                iv: [None; 2],
                ow: d[4],
                ov: None,
            }
        }
    }
    fn clear(&mut self) {
        if !self.op.is_wire() {
            self.iv[0] = None;
            self.iv[1] = None;
        }
        self.ov = None;
    }
}
#[derive(Debug, Clone)]
struct Gates<'a> {
    vec: Vec<Gate<'a>>,
}
impl<'a> Gates<'a> {
    fn new(d: &'a str) -> Self {
        Self {
            vec: d
                .lines()
                .filter(|d| !d.is_empty())
                .map(Gate::new)
                .collect_vec(),
        }
    }
    fn get_ois(&self) -> HashMap<String, usize> {
        self.vec
            .iter()
            .enumerate()
            .map(|(oi, g)| (g.ow.to_string(), oi))
            .collect::<HashMap<_, _>>()
    }
    fn get_iis(&self) -> HashMap<String, Vec<(usize, usize)>> {
        let mut iis = HashMap::new();
        for (ii, g) in self.vec.iter().enumerate() {
            for (iii, &iw) in g.iw.iter().enumerate() {
                iis.entry(iw.to_string())
                    .or_insert(Vec::new())
                    .push((ii, iii));
            }
        }
        iis
    }
    fn resolve_inputs(
        &mut self,
        gi: usize,
        ois: &HashMap<String, usize>,
        iis: &HashMap<String, Vec<(usize, usize)>>,
        max_allowed: &mut usize,
    ) {
        if self.vec[gi].ov.is_some() {
            return;
        }
        let mut pending = vec![gi];
        'out: while let Some(gi) = pending.pop() {
            for iii in 0..2 {
                if self.vec[gi].iv[iii].is_none() && *max_allowed > 0 {
                    pending.push(gi);
                    *max_allowed -= 1;
                    pending.push(ois[self.vec[gi].iw[iii]]);
                    continue 'out;
                }
            }
            if let Some(ov) = self.vec[gi].op.exec(self.vec[gi].iv[0], self.vec[gi].iv[1]) {
                self.vec[gi].ov = Some(ov);
                if iis.contains_key(self.vec[gi].ow) {
                    for &(ii, iii) in iis[self.vec[gi].ow].iter() {
                        self.vec[ii].iv[iii] = Some(ov);
                    }
                }
            }
        }
    }
    fn try_resolve(mut self) -> Option<GatesResolved<'a>> {
        let ois = self.get_ois();
        let iis = self.get_iis();
        let mut max_allowed = self.vec.len() * 16;
        for (ow, oi) in self.get_ois() {
            if iis.contains_key(&ow) {
                continue;
            } else {
                self.resolve_inputs(oi, &ois, &iis, &mut max_allowed);
                if max_allowed == 0 {
                    return None;
                }
            }
        }
        Some(GatesResolved { g: self })
    }
    fn resolve<const DBG_PRINT: bool>(mut self) -> GatesResolved<'a> {
        let ois = self.get_ois();
        let mut pois = self
            .vec
            .iter()
            .enumerate()
            .filter(|(_, g)| g.ov.is_none())
            .map(|(oi, g)| (g.ow, oi))
            .collect::<HashMap<_, _>>();
        let iis = self.get_iis();
        let mut pi = 0;
        let mut loop_count = 0;
        'out: loop {
            if DBG_PRINT {
                loop_count += 1;
                println!("{loop_count}: start");
            }
            for (iw, iv) in self.vec[pi].iw.iter().zip(self.vec[pi].iv.iter()) {
                if iv.is_none() {
                    pi = ois[*iw];
                    if DBG_PRINT {
                        println!("{loop_count}: continue");
                    }
                    continue 'out;
                }
            }
            if let Some(ov) = self.vec[pi].op.exec(self.vec[pi].iv[0], self.vec[pi].iv[1]) {
                self.vec[pi].ov = Some(ov);
                pois.remove(self.vec[pi].ow);
                if iis.contains_key(self.vec[pi].ow) {
                    for &(ii, iii) in iis[self.vec[pi].ow].iter() {
                        self.vec[ii].iv[iii] = Some(ov);
                    }
                }
                if let Some(&npi) = pois.values().next() {
                    pi = npi;
                    if DBG_PRINT {
                        println!("{loop_count}: next");
                    }
                } else {
                    if DBG_PRINT {
                        println!("{loop_count}: end");
                    }
                    return GatesResolved { g: self };
                }
            } else {
                panic!("resolve: this shouldn't happen");
            }
        }
    }
    fn try_get_result(self) -> Option<usize> {
        self.try_resolve().map(|r| r.get_result())
    }
    fn try_build_result(d: &'a str) -> Option<usize> {
        Self::new(d).try_get_result()
    }
    fn set_value(&mut self, s: char, v: usize) {
        self.vec
            .iter_mut()
            .filter(|g| g.ow.starts_with(s))
            .sorted_by_key(|g| g.ow)
            .enumerate()
            .for_each(|(i, g)| {
                let v = Some(((v >> i) & 1) as u8);
                g.iv[0] = v;
                g.iv[1] = v;
            });
    }
    fn set_x(&mut self, v: usize) {
        self.set_value('x', v);
    }
    fn set_y(&mut self, v: usize) {
        self.set_value('y', v);
    }
    fn get_nonwire_gates(&self) -> Vec<usize> {
        self.vec
            .iter()
            .enumerate()
            .filter(|(_, g)| !g.op.is_wire())
            .map(|(i, _)| i)
            .collect_vec()
    }
    fn get_gates_by_zi(&self, zi: usize, c: usize) -> Vec<usize> {
        let mut v = Vec::new();
        let ois = self.get_ois();
        let zgi = ois[&format!("z{zi:02}")];
        v.push(zgi);
        let mut pending = VecDeque::new();
        pending.push_back(zgi);
        let mut rem = c - 1;
        while let Some(i) = pending.pop_front() {
            for ii in self.vec[i].iw.iter().map(|&iw| ois[iw]) {
                if !self.vec[ii].op.is_wire() && rem != 0 {
                    v.push(ii);
                    pending.push_back(ii);
                    rem -= 1;
                }
            }
        }
        v
    }
    fn try_resolve_w_xy(mut self, x: usize, y: usize) -> Option<GatesResolved<'a>> {
        self.set_x(x);
        self.set_y(y);
        self.try_resolve()
    }
}
struct GatesResolved<'a> {
    g: Gates<'a>,
}
impl GatesResolved<'_> {
    fn clone_unresolved(&self) -> Gates {
        let mut g = self.g.clone();
        g.vec.iter_mut().for_each(Gate::clear);
        g
    }
    fn get_value(&self, s: char) -> usize {
        self.g
            .vec
            .iter()
            .filter(|g| g.ow.starts_with(s))
            .sorted_by_key(|&g| g.ow)
            .enumerate()
            .fold(0, |a, (i, g)| {
                a + ((g.ov.expect("get_value") as usize) << i)
            })
    }
    fn get_result(&self) -> usize {
        self.get_z()
    }
    fn get_x(&self) -> usize {
        self.get_value('x')
    }
    fn get_y(&self) -> usize {
        self.get_value('y')
    }
    fn get_z(&self) -> usize {
        self.get_value('z')
    }
    fn get_msb_pos(v: usize) -> Option<usize> {
        const MAX: u32 = usize::BITS;
        let dlz = v.leading_zeros();
        if MAX == dlz {
            None
        } else {
            Some((MAX - dlz - 1) as usize)
        }
    }
    fn get_lowest_bad_zi(&self, max_pos: usize) -> Option<usize> {
        for i in 0..max_pos {
            let mut g = self.clone_unresolved();
            g.set_x(1 << i);
            g.set_y(0);
            let r = g.resolve::<false>();
            let zi = r.get_res_diff::<false>().trailing_zeros();
            if zi != usize::BITS {
                return Some(zi as usize);
            }
        }
        None
    }
    fn get_bad_zis(&self) -> Vec<usize> {
        let mut v = Vec::new();
        for i in 0..usize::BITS {
            let mut g = self.clone_unresolved();
            g.set_x(1 << i);
            g.set_y(0);
            let r = g.try_resolve().unwrap();
            let diff = r.get_res_diff::<false>();
            if diff != 0 {
                v.push(i as usize);
            }
        }
        v
    }
    fn try_get_fixes(&self) -> Vec<String> {
        let mut v = Vec::new();
        let mut res = Vec::new();
        let zis = self.get_bad_zis();
        for &zi in &zis {
            let g = self.clone_unresolved();
            let related_gis = g.get_gates_by_zi(zi, 5);
            for gi in related_gis {
                v.extend(self.try_get_fix_for_gi(zi, gi));
            }
        }
        let combos = v.into_iter().combinations(zis.len()).collect_vec();
        'combo: for combo in combos {
            let counts = combo.iter().flat_map(|(a, b)| [a, b]).counts();
            if !counts.values().all(|v| v == &1) {
                continue;
            }
            let mut g = self.clone_unresolved();
            let ois = g.get_ois();
            for (a, b) in &combo {
                g.vec[ois[a]].ow = b;
                g.vec[ois[b]].ow = a;
            }
            if let Some(r) = g.try_resolve() {
                if r.get_res_diff::<false>() == 0 {
                    for i in 0..usize::BITS {
                        for n in 1..4 {
                            let n = n << i;
                            if r.is_bad_xy(n, n) || r.is_bad_xy(n, n >> 1) {
                                continue 'combo;
                            }
                        }
                    }
                    res.push(
                        combo
                            .into_iter()
                            .flat_map(|(a, b)| [a, b])
                            .sorted()
                            .join(","),
                    );
                }
            }
        }
        res
    }
    fn is_bad_xy(&self, x: usize, y: usize) -> bool {
        if let Some(r) = self.clone_unresolved().try_resolve_w_xy(x, y) {
            if r.get_res_diff::<false>() == 0 {
                return false;
            }
        }
        true
    }
    fn try_get_fix_for_gi(&self, zi: usize, gi: usize) -> Vec<(String, String)> {
        let mut v = Vec::new();
        for nwi in self.g.get_nonwire_gates() {
            if nwi == gi {
                continue;
            }
            let mut g = self.clone_unresolved();
            g.set_x(1 << zi);
            g.set_y(0);
            let ow1 = g.vec[gi].ow;
            let ow2 = g.vec[nwi].ow;
            g.vec[gi].ow = ow2;
            g.vec[nwi].ow = ow1;
            if let Some(r) = g.try_resolve() {
                let diff = r.get_res_diff::<false>();
                if diff == 0 {
                    v.push((ow1.to_string(), ow2.to_string()));
                }
            }
        }
        v
    }
    fn print_bit_header() {
        (0..usize::BITS).rev().for_each(|v| print!("{}", v / 10));
        println!();
        (0..usize::BITS).rev().for_each(|v| print!("{}", v % 10));
        println!();
    }
    fn get_res_diff<const DBG_PRINT: bool>(&self) -> usize {
        let exp = self.get_x() + self.get_y();
        let res = self.get_z();
        let diff = exp ^ res;
        if DBG_PRINT {
            Self::print_bit_header();
            println!("{diff:64b}\n{exp:64b}\n{res:64b}");
        }
        diff
    }
}

fn dump_map(d: &str) -> String {
    //https://mermaid.live/
    let mut s = String::new();
    for l in d.lines().filter(|s| s.contains("->")) {
        let p = l.split(" ").collect_vec();
        s.push_str(&format!("{} --> {}\n", p[0], p[4]));
        s.push_str(&format!("{} --> {}\n", p[2], p[4]));
        s.push_str(&format!("{0} : {1} {0}\n", p[4], p[1]));
    }
    let mut s2 = String::new();
    s2.push_str("stateDiagram-v2\n");
    s2.push_str(&s.lines().sorted().join("\n"));
    s2
}
fn main() {
    let s = Instant::now();
    println!("START");
    println!(
        "{:?}",
        Gates::new(d()).try_resolve().unwrap().try_get_fixes()
    );
    println!("END {:?}", s.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test] fn t_p1_e1() { assert_eq!(Some(4), Gates::try_build_result(e1())); }
    #[test] fn t_p1_e2() { assert_eq!(Some(2024), Gates::try_build_result(e2())); }
    #[test] fn t_p1_d() { assert_eq!(Some(58639252480880), Gates::try_build_result(d())); }
    #[test] fn t_p2_d() { assert_eq!(vec!["bkr,mqh,rnq,tfb,vvr,z08,z28,z39"], Gates::new(d()).try_resolve().unwrap().try_get_fixes()); }
}
