#![allow(dead_code)]
use std::{iter::once, time::Instant};
use itertools::{self, Itertools};

fn s1() -> &'static str {
    "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"
}
fn s2() -> &'static str {
    include_str!("input.txt")
}
fn e1() -> (usize, usize, usize, &'static str) {
    (7, 7, 12, s1())
}
fn e1_map() -> &'static str {
    "\
...#...
..#..#.
....#..
...#..#
..#..#.
.#..#..
#.#....
"
}
fn d1() -> (usize, usize, usize, &'static str) {
    (71, 71, 1024, s2())
}

fn map_to_str(m: &[Vec<char>]) -> String {
    m.iter()
        .flat_map(|r| r.iter().chain(once(&'\n')).copied())
        .collect()
}
fn get_map((w, h, l, b): (usize, usize, usize, &str)) -> Vec<Vec<char>> {
    let mut m = (0..h).map(|_| vec!['.'; w]).collect::<Vec<_>>();
    for (x, y) in b.lines().take(l).map(|p| {
        p.split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap()
    }) {
        m[y][x] = '#';
    }
    m
}
const DIRS: [(usize, usize); 4] = [
    (1, 0),
    (0, 1),
    (usize::MAX, 0),
    (0, usize::MAX),
];
fn is_backwards(&(dx1, dy1): &(usize, usize), &(dx2, dy2): &(usize, usize)) -> bool {
    if (dx1 == 0 && dy1 == 0) || (dx2 == 0 && dy2 == 0) {
        false
    } else {
        (dx1 == dx2 && dy1 != dy2) || (dy1 == dy2 && dx1 != dx2)
    }
}
fn walk_paths(mut m: Vec<Vec<char>>, (x, y): (usize, usize), &(dx, dy): &(usize, usize), s: usize, ss: &mut Vec<Vec<usize>>) -> Vec<Vec<Vec<char>>> {
    let mut v = Vec::new();
    let nx = x.wrapping_add(dx);
    let ny = y.wrapping_add(dy);
    if nx < m.first().unwrap().len() && ny < m.len() && m[ny][nx] != '#' && m[ny][nx] != 'O' && s < ss[ny][nx] {
        m[ny][nx] = 'O';
        ss[ny][nx] = s;
        // if s % 200 == 0 {
        //     println!("{}", map_to_str(&m));
        // }
        if nx == m.first().unwrap().len() - 1 && ny == m.len() - 1 {
            v.push(m);
        } else {
            for d in DIRS.iter().filter(|&dxdy| !is_backwards(dxdy, &(dx, dy))) {
                v.extend(walk_paths(m.clone(), (nx, ny), d, s + 1, ss));
            }
        }
    }
    v
}
fn get_paths(m: &[Vec<char>]) -> Vec<Vec<Vec<char>>> {
    let mut ss = (0..(m.len())).map(|_| vec![usize::MAX; m.first().unwrap().len()]).collect::<Vec<_>>();
    walk_paths(m.to_vec(), (0, 0), &(0, 0), 0, &mut ss)
}
fn get_min_steps(ms: &[Vec<Vec<char>>]) -> usize {
    if let Some(c) = ms.iter().map(|m| m.iter().flatten().counts()[&'O']).min() {
        c - 1
    } else {
        0
    }
}
fn get_min_map(ms: &[Vec<Vec<char>>]) -> Option<String> {
    let min = get_min_steps(ms) + 1;
    if let Some(m) = ms.iter().find(|m| m.iter().flatten().counts()[&'O'] == min) {
        return Some(map_to_str(m));
    }
    None
}
fn find_when_blocked(d: (usize, usize, usize, &str)) -> &str {
    let mut d = d;
    let m = d.3.lines().count() + 1;
    for c in (0..m).rev() {
        // let st = Instant::now();
        // print!("{c} ");
        d.2 = c;
        let m = &get_map(d);         // does take(c); take(2) would yield nth(0) and nth(1)
        let ms = &get_paths(m);
        let ss = get_min_steps(ms);
        // println!("{s} {:?}", st.elapsed());
        // if let Some(str) = get_min_map(ms) {
        //     print!("{str}");
        // }
        if ss != 0 {
            return d.3.lines().nth(c).unwrap();       // if c == 2 like in comment above, nth(2) would yield the line after nth(0) and nth(1) (the bad line)
        }
    }
    unreachable!()
}
fn main() {
    let s = Instant::now();
    println!("{} {:?}", find_when_blocked(d1()), s.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test] fn is_backwards_0_0_ok() { assert_eq!(DIRS.to_vec(), DIRS.iter().copied().filter(|dxdy| !is_backwards(dxdy, &(0, 0))).collect::<Vec<_>>()) }
    #[test] fn e1_map_ok() { assert_eq!(e1_map(), map_to_str(&get_map(e1()))) }
    #[test] fn e1_steps_ok() { assert_eq!(22, get_min_steps(&get_paths(&get_map(e1())))) }
    #[test] fn d1_steps_ok() { assert_eq!(404, get_min_steps(&get_paths(&get_map(d1())))) }
    #[test] fn d1_blocker() { assert_eq!("27,60", find_when_blocked(d1())) }
}
