#![allow(dead_code)]

use std::{collections::HashMap, time::Instant};
use itertools::Itertools;

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

#[derive(Debug)]
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
}
#[derive(Debug)]
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
}
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
    fn resolve<const DBG_PRINT: bool>(mut self) -> GatesResolved<'a> {
        let ois = self
            .vec
            .iter()
            .enumerate()
            .map(|(oi, g)| (g.ow, oi))
            .collect::<HashMap<_, _>>();
        let mut pois = self
            .vec
            .iter()
            .enumerate()
            .filter(|(_, g)| g.ov.is_none())
            .map(|(oi, g)| (g.ow, oi))
            .collect::<HashMap<_, _>>();
        let mut iis = HashMap::new();
        for (ii, g) in self.vec.iter().enumerate() {
            for (iii, &iw) in g.iw.iter().enumerate() {
                iis.entry(iw).or_insert(Vec::new()).push((ii, iii));
            }
        }
        let mut pi = 0;
        let mut loop_count = 0;
        'out: loop {
            if DBG_PRINT {
                loop_count += 1;
                println!("{loop_count}: start");
            }
            for (iw, iv) in self.vec[pi].iw.iter().zip(self.vec[pi].iv.iter()) {
                if iv.is_none() {
                    pi = ois[iw];
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
                    return GatesResolved { vec: self.vec };
                }
            } else {
                panic!("resolve: this shouldn't happen");
            }
        }
    }
    fn get_result(self) -> usize {
        self.resolve::<false>().get_result()
    }
    fn build_result(d: &'a str) -> usize {
        Self::new(d).get_result()
    }
}
struct GatesResolved<'a> {
    vec: Vec<Gate<'a>>,
}
impl GatesResolved<'_> {
    fn get_result(&self) -> usize {
        self.vec
            .iter()
            .filter(|g| g.ow.starts_with('z'))
            .sorted_by_key(|&g| g.ow)
            .enumerate()
            .fold(0, |a, (i, g)| {
                a + ((g.ov.expect("get_result") as usize) << i)
            })
    }
}

fn main() {
    let s = Instant::now();
    println!("START");
    let g = Gates::new(d());
    let g = g.resolve::<false>();
    println!("{}", g.get_result());
    println!("END {:?}", s.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test] fn t_p1_e1() { assert_eq!(4, Gates::build_result(e1())); }
    #[test] fn t_p1_e2() { assert_eq!(2024, Gates::build_result(e2())); }
    #[test] fn t_p1_d() { assert_eq!(58639252480880, Gates::build_result(d())); }
}
