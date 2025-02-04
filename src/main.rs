#![allow(dead_code, unused_imports)]
use itertools::{self, Itertools};
use std::{collections::HashMap, time::Instant};

fn ex1() -> &'static str {
    "125 17"
}
fn data() -> &'static str {
    "510613 358 84 40702 4373582 2 0 1584"
}
struct Stones {
    line: Vec<usize>,
    cache: HashMap<(usize, usize), usize>,
}
impl Stones {
    const CACHE0: [usize; 48] = [
        1, 1, 1, 2, 4, 4, 7, 14, 16, 20, 39, 62, 81, 110, 200, 328, 418, 667, 1059, 1546, 2377,
        3572, 5602, 8268, 12343, 19778, 29165, 43726, 67724, 102131, 156451, 234511, 357632,
        549949, 819967, 1258125, 1916299, 2886408, 4414216, 6669768, 10174278, 15458147, 23333796,
        35712308, 54046805, 81997335, 125001266, 189148778,
    ];
    const CACHE1: [usize; 48] = [
        1, 1, 2, 4, 4, 7, 14, 16, 20, 39, 62, 81, 110, 200, 328, 418, 667, 1059, 1546, 2377, 3572,
        5602, 8268, 12343, 19778, 29165, 43726, 67724, 102131, 156451, 234511, 357632, 549949,
        819967, 1258125, 1916299, 2886408, 4414216, 6669768, 10174278, 15458147, 23333796,
        35712308, 54046805, 81997335, 125001266, 189148778, 288114305,
    ];
    const CACHE2: [usize; 48] = [
        1, 1, 2, 4, 4, 6, 12, 16, 19, 30, 57, 92, 111, 181, 295, 414, 661, 977, 1501, 2270, 3381,
        5463, 7921, 11819, 18712, 27842, 42646, 64275, 97328, 150678, 223730, 343711, 525238,
        784952, 1208065, 1824910, 2774273, 4230422, 6365293, 9763578, 14777945, 22365694, 34205743,
        51643260, 78678894, 119550250, 181040219, 276213919,
    ];
    fn new(d: &str) -> Stones {
        let mut ss = Stones {
            line: d.split(" ").map(|c| c.parse().expect("parse")).collect(),
            cache: HashMap::new(),
        };
        ss.line.reserve(2_000_000);
        ss
    }
    fn blink_stone(s: usize) -> Vec<usize> {
        if s == 0 {
            vec![1]
        } else {
            let d = s.ilog10() + 1;
            if d % 2 == 0 {
                let mag = 10_usize.pow(d / 2);
                let left = s / mag;
                let right = s - left * mag;
                vec![left, right]
            } else {
                vec![s * 2024]
            }
        }
    }
    fn blink_once(&mut self) {
        let mut inss = vec![];
        for i in 0..self.line.len() {
            match Self::blink_stone(self.line[i])[..] {
                [] => panic!("cannot have none"),
                [r] => self.line[i] = r,
                [r, ins] => {
                    self.line[i] = r;
                    inss.push((i + 1, ins));
                }
                [..] => panic!("cannot have more than 2"),
            }
        }
        while let Some((i, ins)) = inss.pop() {
            self.line.insert(i, ins);
        }
    }
    fn blink_n(&mut self, n: usize) {
        for _ in 0..n {
            self.blink_once();
        }
    }
    fn blink_n_no_mut_step(s: usize, i: usize) -> usize {
        if i == 0 {
            1
        } else {
            Self::blink_stone(s)
                .into_iter()
                .map(|s| Self::blink_n_no_mut_step(s, i - 1))
                .sum()
        }
    }
    fn blink_n_no_mut_count(&self, n: usize) -> usize {
        self.line
            .iter()
            .map(|s| Self::blink_n_no_mut_step(*s, n))
            .sum()
    }
    fn blink_n_no_mut2_step(&mut self, s: usize, i: usize) -> usize {
        if i == 0 {
            1
        } else {
            let i = i - 1;
            let k = (i, s);
            if let Some(v) = self.cache.get(&k) {
                *v
            } else {
                let v = Self::blink_stone(s)
                    .into_iter()
                    .map(|s| self.blink_n_no_mut2_step(s, i))
                    .sum();
                self.cache.insert(k, v);
                v
            }
        }
    }
    fn blink_n_no_mut2_count(&mut self, n: usize) -> usize {
        self.line
            .clone()
            .into_iter()
            .map(|s| self.blink_n_no_mut2_step(s, n))
            .sum()
    }
    fn blink_n_no_mut_iter_count(&self, n: usize) -> usize {
        let mut sum = 0;
        let mut v = self.line.iter().map(|s| (n, *s)).rev().collect::<Vec<_>>();
        v.reserve(4_000_000);
        while let Some((i, s)) = v.pop() {
            if i == 0 {
                sum += 1;
            } else {
                let ss = Self::blink_stone(s);
                if ss.len() == 2 {
                    v.push((i - 1, ss[1]));
                }
                v.push((i - 1, ss[0]));
            }
        }
        sum
    }
    fn blink_n_no_mut_iter2_count(&self, n: usize) -> usize {
        let mut sum = 0;
        let mut v = self.line.iter().map(|s| (n, *s)).rev().collect::<Vec<_>>();
        v.reserve(4_000_000);
        while let Some((i, s)) = v.pop() {
            if s == 0 && i < Self::CACHE0.len() {
                sum += Self::CACHE0[i];
            } else if s == 1 && i < Self::CACHE1.len() {
                sum += Self::CACHE1[i];
            } else if s == 2 && i < Self::CACHE2.len() {
                sum += Self::CACHE2[i];
            } else {
                let mut i = i;
                let mut s = s;
                while i > 0 {
                    let ss = Self::blink_stone(s);
                    if ss.len() == 2 {
                        v.push((i - 1, ss[1]));
                    }
                    s = ss[0];
                    i -= 1;
                }
                sum += 1;
            }
        }
        sum
    }
}
impl std::fmt::Display for Stones {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.line)
    }
}
fn p1() -> usize {
    let mut s = Stones::new(data());
    s.blink_n(25);
    s.line.len()
}
fn p2() -> usize {
    // Stones::new(data()).blink_n_no_mut_iter2_count(25)   //     137.1557ms            217812
    // Stones::new(data()).blink_n_no_mut_iter_count(25)    //     149.3374ms
    // Stones::new(data()).blink_n_no_mut_count(25)         //     242.4749ms
    // Stones::new(data()).blink_n_no_mut_iter_count(30)    //   1.2141807s
    // Stones::new(data()).blink_n_no_mut_count(30)         //   1.4690755s
    // Stones::new(data()).blink_n_no_mut_count(31)         //   2.0538061s
    // Stones::new(data()).blink_n_no_mut_count(32)         //   3.1968968s
    // Stones::new(data()).blink_n_no_mut_count(33)         //   8.884926s
    // Stones::new(data()).blink_n_no_mut_iter_count(34)    //   6.0512944s             9333351
    // Stones::new(data()).blink_n_no_mut_iter2_count(34)   //   6.0695536s             9333351
    Stones::new(data()).blink_n_no_mut2_count(75)        //  12.6368316s     259112729857522
    // Stones::new(data()).blink_n_no_mut_count(34)         //  13.2555812s
    // Stones::new(data()).blink_n_no_mut_iter_count(40)    //  74.4118509s
    // Stones::new(data()).blink_n_no_mut_count(40)         // 125.1675894s
    // let mut s = Stones::new(data());
    // let r = s.blink_n_no_mut2_count(75);
    // // for (count, val) in s
    // //     .cache
    // //     .iter()
    // //     .map(|((i, s), v)| *s)
    // //     .sorted()
    // //     .dedup_with_count()
    // //     .sorted_by_key(|(c, v)| *c)
    // // {
    // //     println!("count {count} val {val}"); 
    // // }
    // r
}
fn main() {
    let s = Instant::now();
    println!("{} {:?}", p2(), s.elapsed());
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn p1_ex1() {
        let mut s = Stones::new(ex1());
        assert_eq!("[125, 17]", s.to_string());
        s.blink_once();
        assert_eq!("[253000, 1, 7]", s.to_string());
        s.blink_once();
        assert_eq!("[253, 0, 2024, 14168]", s.to_string());
        s.blink_once();
        assert_eq!("[512072, 1, 20, 24, 28676032]", s.to_string());
        s.blink_once();
        assert_eq!("[512, 72, 2024, 2, 0, 2, 4, 2867, 6032]", s.to_string());
        s.blink_once();
        assert_eq!(
            "[1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]",
            s.to_string()
        );
        s.blink_once();
        assert_eq!("[2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2]", s.to_string());
    }
    #[test]
    fn p1_ex1_len() {
        let mut s = Stones::new(ex1());
        s.blink_n(25);
        assert_eq!(55312, s.line.len());
    }
    #[test]
    fn p1_test() {
        assert_eq!(217812, p1());
    }
    #[test]
    fn p2_test_1() {
        assert_eq!(217812, Stones::new(data()).blink_n_no_mut_count(25));
    }
    #[test]
    fn p2_test_2() {
        assert_eq!(217812, Stones::new(data()).blink_n_no_mut_iter_count(25));
    }
    #[test]
    fn p2_test_3() {
        assert_eq!(217812, Stones::new(data()).blink_n_no_mut_iter2_count(25));
    }
    #[test]
    fn p2_test_4() {
        assert_eq!(217812, Stones::new(data()).blink_n_no_mut2_count(25));
    }
    #[test]
    fn p2_test() {
        assert_eq!(259112729857522, p2());
    }
}
