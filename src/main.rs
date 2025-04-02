#![allow(dead_code)]
use std::collections::HashSet;
use std::collections::HashMap;
fn ez1() -> &'static str {
    "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
"
}
fn ez2() -> &'static str {
    "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
"
}
fn ez3() -> &'static str {
    "\
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
"
}
fn ex() -> &'static str {
    "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"
}
fn data() -> &'static str {
    include_str!("input.txt")
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
struct Map {
    locs: Vec<Vec<char>>,
    trailheads: Vec<Point>,
}
impl Map {
    fn next_valid_step(c: &char) -> Option<char> {
        match c {
            ' ' => Some('0'),
            '0' => Some('1'),
            '1' => Some('2'),
            '2' => Some('3'),
            '3' => Some('4'),
            '4' => Some('5'),
            '5' => Some('6'),
            '6' => Some('7'),
            '7' => Some('8'),
            '8' => Some('9'),
            '9' => None,
            _ => None,
        }
    }
    const DIRECTIONS: [(usize, usize); 4] = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    fn new(d: &str) -> Self {
        let mut locs = Vec::new();
        let mut trailheads = Vec::new();
        for (y, l) in d.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in l.chars().enumerate() {
                row.push(c);
                if c == '0' {
                    trailheads.push(Point::new(x, y));
                }
            }
            locs.push(row);
        }
        Map { locs, trailheads }
    }
    fn get(&self, &Point { x, y }: &Point) -> Option<&char> {
        self.locs.get(y)?.get(x)
    }
    fn step(&self, point: &Point, pre: char) -> Option<Vec<Point>> {
        let cur = self.get(point)?;
        let ver = Self::next_valid_step(&pre)?;
        let nex = Self::next_valid_step(cur);
        if *cur != ver {
            None
        } else if nex.is_none() {
            Some(vec![*point])
        } else {
            let v = Self::DIRECTIONS
                .iter()
                .map(|(dx, dy)| Point::new(point.x.wrapping_add(*dx), point.y.wrapping_add(*dy)))
                .filter_map(|p| self.step(&p, *cur))
                .flatten()
                .collect::<Vec<_>>();
            if v.is_empty() {
                None
            } else {
                Some(v)
            }
        }
    }
    fn get_trailhead_9s_reached(&self, point: &Point) -> (Point, Vec<Point>, HashSet<Point>) {
        let v = self.step(point, ' ').unwrap_or_default();
        let h = HashSet::from_iter(v.iter().cloned());
        (*point, v, h)
    }
    fn get_trailheads_9s_reached(&self) -> HashMap<Point, (Point, Vec<Point>, HashSet<Point>)> {
        self.trailheads
            .iter()
            .map(|point| {
                (*point, self.get_trailhead_9s_reached(point))
            })
            .collect()
    }
    fn get_trailhead_score(&self, point: &Point) -> usize {
        self.get_trailhead_9s_reached(point).2.len()
    }
    fn get_trailheads_score(&self) -> usize {
        self.get_trailheads_9s_reached().iter().fold(0, |a, (_, set)| a + set.2.len())
    }
    fn get_trailhead_rating(&self, point: &Point) -> usize {
        self.get_trailhead_9s_reached(point).1.len()
    }
    fn get_trailheads_rating(&self) -> usize {
        self.get_trailheads_9s_reached().iter().fold(0, |a, (_, set)| a + set.1.len())
    }
}
fn main() {
    let m = Map::new(data());
    println!("{:?}", m.get_trailheads_rating());
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn ex_get() {
        let m = Map::new(ex());
        assert_eq!(m.get(&Point::new(0, 0)), Some(&'8'));
        assert_eq!(m.get(&Point::new(1, 0)), Some(&'9'));
        assert_eq!(m.get(&Point::new(2, 0)), Some(&'0'));
        assert_eq!(m.get(&Point::new(3, 0)), Some(&'1'));
        assert_eq!(m.get(&Point::new(4, 0)), Some(&'0'));
        assert_eq!(m.get(&Point::new(8, 0)), None);
        assert_eq!(m.get(&Point::new(7, 7)), Some(&'2'));
        assert_eq!(m.get(&Point::new(0, 8)), None);
    }
    #[test] fn p1_ez1() { assert_eq!(2, Map::new(ez1()).get_trailheads_score()); }
    #[test] fn p1_ez2() { assert_eq!(4, Map::new(ez2()).get_trailheads_score()); }
    #[test] fn p1_ez3() { assert_eq!(3, Map::new(ez3()).get_trailheads_score()); }
    #[test] fn p1_ex() { assert_eq!(36, Map::new(ex()).get_trailheads_score()); }
    #[test] fn p1_data() { assert_eq!(719, Map::new(data()).get_trailheads_score()); }
    #[test] fn p2_ex() { assert_eq!(81, Map::new(ex()).get_trailheads_rating()); }
    #[test] fn p2_data() { assert_eq!(1530, Map::new(data()).get_trailheads_rating()); }
}
