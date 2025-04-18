#![allow(dead_code)]

use itertools::Itertools;
use std::{collections::HashMap, ops::Add};
fn ez1() -> &'static str {
    "\
A
"
}
fn ez1a() -> &'static str {
    "\
AA
"
}
fn ez2() -> &'static str {
    "\
AA
AA
"
}
fn ex1() -> &'static str {
    "\
AAAA
BBCD
BBCC
EEEC
"
}
fn ex2() -> &'static str {
    "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"
}
fn ex3() -> &'static str {
    "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"
}
fn p2_ex1() -> &'static str {
    "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"
}
fn p2_ex2() -> &'static str {
    "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"
}
fn data() -> &'static str {
    include_str!("input.txt")
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}
impl Add for Point {
    type Output = Point;
    fn add(self, p: Point) -> <Self as std::ops::Add<Point>>::Output {
        Point {
            x: self.x.wrapping_add(p.x),
            y: self.y.wrapping_add(p.y),
        }
    }
}
impl Point {
    const N1: usize = usize::MAX;
    const N2: usize = Self::N1 - 1;
    const RIGHT1: Self = Self { x: 1, y: 0 };
    const DOWN1: Self = Self { x: 0, y: 1 };
    const LEFT1: Self = Self { x: Self::N1, y: 0 };
    const UP1: Self = Self { x: 0, y: Self::N1 };
    const DIRECTIONS1: [Self; 4] = [Self::RIGHT1, Self::DOWN1, Self::LEFT1, Self::UP1];
    const RIGHT2: Self = Self { x: 2, y: 0 };
    const DOWN2: Self = Self { x: 0, y: 2 };
    const LEFT2: Self = Self { x: Self::N2, y: 0 };
    const UP2: Self = Self { x: 0, y: Self::N2 };
    const DIRECTIONS2: [Self; 4] = [Self::RIGHT2, Self::DOWN2, Self::LEFT2, Self::UP2];

    const DR1: Self = Self { x: 1, y: 1 };
    const DL1: Self = Self { x: Self::N1, y: 1 };
    const UR1: Self = Self { x: 1, y: Self::N1 };
    const UL1: Self = Self {
        x: Self::N1,
        y: Self::N1,
    };

    fn half2s(&self) -> &Self {
        match *self {
            Self::RIGHT2 => &Self::RIGHT1,
            Self::DOWN2 => &Self::DOWN1,
            Self::LEFT2 => &Self::LEFT1,
            Self::UP2 => &Self::UP1,
            _ => todo!(),
        }
    }
    fn fence_kind(&self) -> char {
        match *self {
            Self::RIGHT2 => '|',
            Self::DOWN2 => '_',
            Self::LEFT2 => '|',
            Self::UP2 => '_',
            _ => todo!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Space {
    kind: char,
    visited: bool,
}
impl Space {
    fn new(c: char) -> Self {
        Space {
            kind: c,
            visited: false,
        }
    }
}
#[derive(Clone)]
struct Map {
    spaces: Vec<Vec<Space>>,
}
impl Map {
    fn verify_row_lens(m: Self) -> Self {
        assert!(m.spaces.iter().is_sorted_by(|a, b| a.len() == b.len()));
        m
    }
    fn new(d: &str) -> Self {
        let mut m = Map { spaces: Vec::new() };
        for l in d.lines() {
            let mut fence = Vec::new();
            let mut row = Vec::new();
            for c in l.chars() {
                fence.push(Space::new(' '));
                fence.push(Space::new(' '));
                row.push(Space::new(' '));
                row.push(Space::new(c));
            }
            fence.push(Space::new(' '));
            row.push(Space::new(' '));
            m.spaces.push(fence);
            m.spaces.push(row);
        }
        m.spaces.push(m.spaces[0].to_vec());
        Self::verify_row_lens(m)
    }
    fn step_region(
        t: &mut Vec<Vec<Space>>,
        r: (char, usize),
        m: &mut Vec<Vec<Space>>,
        p: Point,
        d: Point,
    ) {
        let n = p + d;
        let f = p + *Point::half2s(&d);
        let max_x = m[0].len();
        let max_y = m.len();
        if n.x >= max_x || n.y >= max_y {
            m[f.y][f.x].kind = Point::fence_kind(&d);
        } else {
            let ns = &mut m[n.y][n.x];
            if ns.kind != r.0 {
                m[f.y][f.x].kind = Point::fence_kind(&d);
            } else if !ns.visited {
                ns.visited = true;
                t[n.y][n.x].visited = true;
                for d in Point::DIRECTIONS2 {
                    Self::step_region(t, r, m, n, d);
                }
            }
        }
        //first                         no mod point    no fence       recurse  no check
        //out of bounds:                   mod point   set fence    no recurse
        //wrong kind:                      mod point   set fence    no recurse
        //visited:                         mod point    no fence    no recurse
        //not vis, inbnd, right kind       mod point    no fence       recurse
    }
    fn walk_region(&self, t: &mut Vec<Vec<Space>>, r: (char, usize), p: Point) -> Vec<Vec<Space>> {
        let mut m = self.spaces.clone();
        for d in Point::DIRECTIONS2 {
            m[p.y][p.x].visited = true;
            t[p.y][p.x].visited = true;
            Self::step_region(t, r, &mut m, p, d);
        }
        m
    }
    fn walk_all_regions(&mut self) -> HashMap<(char, usize), Vec<Vec<Space>>> {
        let mut h = HashMap::new();
        let mut i = 0;
        let mut t = self.spaces.clone();
        for y in (1..t.len()).step_by(2) {
            for x in (1..t[0].len()).step_by(2) {
                let s = t[y][x];
                if !s.visited {
                    let k = (s.kind, i);
                    let m = self.walk_region(&mut t, k, Point { x, y });
                    h.insert(k, m);
                    i += 1;
                }
            }
        }
        h
    }
    fn get_map_str(v: &[Vec<Space>], sv: bool) -> String {
        let vd = if sv { "v" } else { "" };
        let nv = if sv { " " } else { "" };
        v.iter()
            .map(|r| {
                format!(
                    "{}\n",
                    r.iter()
                        .map(|c| format!("{}{}", c.kind, if c.visited { vd } else { nv }))
                        .join("")
                )
            })
            .join("")
    }
    fn get_region_price(((_r, _), m): (&(char, usize), &Vec<Vec<Space>>)) -> usize {
        let a = m
            .iter()
            .flat_map(|r| r.iter().map(|c| c.visited as usize))
            .sum::<usize>();
        let p = m
            .iter()
            .flat_map(|r| r.iter().map(|c| (c.kind == '|' || c.kind == '_') as usize))
            .sum::<usize>();
        p * a
    }
    fn get_regions_price(m: &HashMap<(char, usize), Vec<Vec<Space>>>) -> usize {
        m.iter().map(Self::get_region_price).sum()
    }
    fn find_first_part_of_fence(m: &[Vec<Space>]) -> Option<Point> {
        for (y, r) in m.iter().enumerate() {
            for (x, c) in r.iter().enumerate() {
                if c.kind == '|' || c.kind == '_' {
                    return Some(Point { x, y });
                }
            }
        }
        None
    }
    fn get_next_fence_dirs(k: char) -> [Point; 6] {
        if k == '_' {
            [
                Point::RIGHT2,
                Point::DR1,
                Point::DL1,
                Point::LEFT2,
                Point::UL1,
                Point::UR1,
            ]
        } else if k == '|' {
            [
                Point::DOWN2,
                Point::DL1,
                Point::UL1,
                Point::UP2,
                Point::UR1,
                Point::DR1,
            ]
        } else {
            panic!("get_next_fence_dirs")
        }
    }

    //WRONG
    fn get_region_price_p2(((_r, _), m): (&(char, usize), &Vec<Vec<Space>>)) -> usize {
        let a = m
            .iter()
            .flat_map(|r| r.iter().map(|c| c.visited as usize))
            .sum::<usize>();
        let start = Self::find_first_part_of_fence(m).expect("find_first_part_of_fence");
        let mut last_side = m[start.y][start.x].kind;
        let mut prev = start;
        let mut curr = start;
        let mut sides = 0;
        loop {
            let next = Self::get_next_fence_dirs(last_side)
                .iter()
                .map(|d| curr + *d)
                .find(|&maybe| {
                    if maybe.y >= m.len() || maybe.x >= m[0].len() {
                        return false;
                    }
                    let kind = m[maybe.y][maybe.x].kind;
                    maybe != prev && (kind == '|' || kind == '_')
                })
                .expect("get_region_price_p2");
            // println!("{next:?}");
            let sd = m[next.y][next.x].kind;
            if sd != last_side {
                sides += 1;
                last_side = sd;
            }
            prev = curr;
            curr = next;
            if curr == start {
                break;
            }
        }

        sides * a
    }

    fn get_region_price_p2_2(((_r, _i), m): (&(char, usize), &Vec<Vec<Space>>)) -> usize {
        let a = m
            .iter()
            .flat_map(|r| r.iter().map(|c| c.visited as usize))
            .sum::<usize>();
        let max_y = m.len();
        let max_x = m[0].len();

        let mut hor = 0;
        for y in (0..max_y).step_by(2) {
            let mut last_was_a_side = false;
            for x in (1..max_x).step_by(2) {
                if m[y][x].kind == '_' {
                    if !last_was_a_side {
                        hor += 1;
                    }
                    let up_y = y.wrapping_add(usize::MAX);
                    let do_y = y + 1;
                    if up_y < max_y && do_y < max_y {
                        let ri_x = x + 1;
                        if m[up_y][ri_x].kind == '|' && m[do_y][ri_x].kind == '|' {
                            hor += 1;
                        }
                    }
                    last_was_a_side = true;
                } else {
                    last_was_a_side = false;
                }
            }
        }
        let mut ver = 0;
        for x in (0..max_x).step_by(2) {
            let mut last_was_a_side = false;
            for y in (1..max_y).step_by(2) {
                if m[y][x].kind == '|' {
                    if !last_was_a_side {
                        ver += 1;
                    }
                    let le_x = x.wrapping_add(usize::MAX);
                    let ri_x = x + 1;
                    if le_x < max_x && ri_x < max_x {
                        let do_y = y + 1;
                        if m[do_y][le_x].kind == '_' && m[do_y][ri_x].kind == '_' {
                            ver += 1;
                        }
                    }
                    last_was_a_side = true;
                } else {
                    last_was_a_side = false;
                }
            }
        }
        // println!("{_r}:{_i} area {a:2} hor {hor} ver {ver} tot {}", (hor + ver) * a);
        (hor + ver) * a
    }
    fn get_regions_price_p2(m: &HashMap<(char, usize), Vec<Vec<Space>>>) -> usize {
        m.iter().map(Self::get_region_price_p2_2).sum()
    }
}
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", Self::get_map_str(&self.spaces, false))
    }
}
fn main() {
    println!("{}", Map::get_regions_price_p2(&Map::new(data()).walk_all_regions()));
}
#[cfg(test)]
mod test {
    use super::*;
    
    #[test] fn ez1_price() { assert_eq!(4, Map::get_regions_price(&Map::new(ez1()).walk_all_regions())); }
    #[test] fn ez1a_price() { assert_eq!(12, Map::get_regions_price(&Map::new(ez1a()).walk_all_regions())); }
    #[test] fn ez2_price() { assert_eq!(32, Map::get_regions_price(&Map::new(ez2()).walk_all_regions())); }
    #[test] fn ex1_price() { assert_eq!(140, Map::get_regions_price(&Map::new(ex1()).walk_all_regions())); }
    #[test] fn ex2_price() { assert_eq!(772, Map::get_regions_price(&Map::new(ex2()).walk_all_regions())); }
    #[test] fn ex3_price() { assert_eq!(1930, Map::get_regions_price(&Map::new(ex3()).walk_all_regions())); }
    #[test] fn p1_price() { assert_eq!(1473408, Map::get_regions_price(&Map::new(data()).walk_all_regions())); }
    #[test] fn ez1_price2() { assert_eq!(4, Map::get_regions_price_p2(&Map::new(ez1()).walk_all_regions())); }
    #[test] fn ez1a_price2() { assert_eq!(8, Map::get_regions_price_p2(&Map::new(ez1a()).walk_all_regions())); }
    #[test] fn ez2_price2() { assert_eq!(16, Map::get_regions_price_p2(&Map::new(ez2()).walk_all_regions())); }
    #[test] fn ex1_price2() { assert_eq!(80, Map::get_regions_price_p2(&Map::new(ex1()).walk_all_regions())); }
    #[test] fn ex_p2_1() { assert_eq!(236, Map::get_regions_price_p2(&Map::new(p2_ex1()).walk_all_regions())); }
    #[test] fn ex_p2_2() { assert_eq!(368, Map::get_regions_price_p2(&Map::new(p2_ex2()).walk_all_regions())); }
    #[test] fn ex2_price2() { assert_eq!(436, Map::get_regions_price_p2(&Map::new(ex2()).walk_all_regions())); }
    #[test] fn ex3_price2() { assert_eq!(1206, Map::get_regions_price_p2(&Map::new(ex3()).walk_all_regions())); }
    #[test] fn p2_price() {  assert_eq!(886364, Map::get_regions_price_p2(&Map::new(data()).walk_all_regions()));  }
    
    #[test]
    fn ex3_price2_prints() {
        let mut m = Map::new(data());
        let h = &m.walk_all_regions();
        // for (r, m) in h {
        //     println!("{r:?}");
        //     println!("{}", Map::get_map_str(m, false));
        // }
        // println!("{}", Map::get_regions_price_p2(h));
        assert_eq!(
            886364,
            Map::get_regions_price_p2(h)
        );
    }
}
