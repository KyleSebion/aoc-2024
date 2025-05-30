#![allow(dead_code)]
use std::{collections::VecDeque, iter::{self, once}, sync::LazyLock, time::{Duration, Instant}};
use itertools::{self, Itertools};
fn e1() -> &'static str {
    "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"
}
fn e2() -> &'static str {
    "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"
}
fn d() -> &'static str {
    include_str!("input.txt")
}
enum Kind {
    Empty,
    Wall,
    Start,
    End,
}
struct Space {
    k: Kind,
    c: usize,
}
impl Space {
    fn new(d: char) -> Self {
        let (k, c) = match d {
            '.' => (Kind::Empty, usize::MAX),
            '#' => (Kind::Wall, 0),
            'S' => (Kind::Start, 0),
            'E' => (Kind::End, usize::MAX),
            _ => panic!("wrong kind"),
        };
        Space { k, c }
    }
    fn to_char(&self) -> char {
        match self.k {
            Kind::Empty => '.',
            Kind::Wall => '#',
            Kind::Start => 'S',
            Kind::End => 'E',
        }
    }
}
impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Pos {
    x: usize,
    y: usize,
}
#[derive(PartialEq, Eq, Clone, Debug)]
struct Dir {
    dx: usize,
    dy: usize,
}
impl Dir {
    const RIGHT: Self = Self::new(1, 0);
    const LEFT: Self = Self::new(usize::MAX, 0);
    const DOWN: Self = Self::new(0, 1);
    const UP: Self = Self::new(0, usize::MAX);
    const RDLU: [Self; 4] = [Self::RIGHT, Self::DOWN, Self::LEFT, Self::UP];
    const fn new(dx: usize, dy: usize) -> Self {
        Self { dx, dy }
    }
    fn is_180(&self, d: &Dir) -> bool {
        (self.dy == 0 && d.dy == 0 && self.dx != d.dx)
            || (self.dx == 0 && d.dx == 0 && self.dy != d.dy)
    }
}
#[derive(Clone, Debug)]
struct Reindeer {
    p: Pos,
    d: Dir,
    t: usize,
    s: usize,
    start: Instant,
    dur: Duration,
    path: Vec<Pos>,
}
static START: LazyLock<Instant> = LazyLock::new(Instant::now);
impl Reindeer {
    fn new(p: &Pos) -> Self {
        let mut s =        Self {
            p: Pos { x: p.x, y: p.y },
            d: Dir::RIGHT,
            t: 0,
            s: 0,
            start: *START,
            dur: Duration::ZERO,
            path: Vec::new(),
        };
        s.path.push(s.p.clone());
        s
    }
    fn cost(&self) -> usize {
        self.t * 1000 + self.s
    }
}
type Vrd = VecDeque<(Reindeer, bool)>;
struct Map {
    m: Vec<Vec<Space>>,
    s: Pos,
    e: Pos,
}
impl Map {
    fn new(d: &str) -> Self {
        let mut m = Self {
            m: Vec::new(),
            s: Pos { x: 0, y: 0 },
            e: Pos { x: 0, y: 0 },
        };
        for (y, l) in d.lines().enumerate() {
            let mut r = Vec::new();
            for (x, c) in l.char_indices() {
                let s = Space::new(c);
                if let Kind::Start = s.k {
                    m.s.x = x;
                    m.s.y = y;
                }
                if let Kind::End = s.k {
                    m.e.x = x;
                    m.e.y = y;
                }
                r.push(s);
            }
            m.m.push(r);
        }
        assert!(m.m.iter().is_sorted_by(|a, b| a.len() == b.len()));
        m
    }
    fn map_str(&self) -> String {
        let mut s = String::new();
        for r in self.m.iter() {
            for c in r.iter().map(Space::to_char) {
                s.push(c);
            }
            s.push('\n');
        }
        s
    }
    fn map_str_w_r(&self, rs: &Vrd) -> String {
        let mut s = String::new();
        for (y, r) in self.m.iter().enumerate() {
            for (x, c) in r.iter().map(Space::to_char).enumerate() {
                let c = if rs.iter().any(|(r, _)| r.p.x == x && r.p.y == y)
                    && (Pos { x, y }) != self.e
                {
                    'R'
                } else {
                    c
                };
                s.push(c);
            }
            s.push('\n');
        }
        s
    }
    fn map_str_w_r_cost(&self, rs: &Vrd) -> String {
        let mut s = String::new();
        for (y, r) in self.m.iter().enumerate() {
            for (x, c) in r.iter().map(Space::to_char).enumerate() {
                const W: usize = 7;
                let min_rs = rs.iter().filter(|(r, _)| r.p.x == x && r.p.y == y).min_by_key(|(r, _)| r.cost());
                let c = if let Some((r, _)) = min_rs {
                    format!("{:^W$}", r.cost())
                } else {
                    String::from_iter(iter::repeat(c).take(W))
                };
                s.push_str(&c);
            }
            s.push('\n');
        }
        s
    }
    fn colored_map_str_w_r(&self, rs: &Vrd) -> String {
        let mut s = String::new();
        for (y, r) in self.m.iter().enumerate() {
            for (x, sp) in r.iter().enumerate() {
                let mut c = sp.to_char();
                if sp.c != 0 && sp.c != usize::MAX {
                    c = '*';
                }
                let str = if rs.iter().any(|(r, _)| r.p.x == x && r.p.y == y)
                    && (Pos { x, y }) != self.e
                {
                    "\x1b[38;2;255;0;0m▉▉\x1b[38;2;255;255;255m".to_string()
                } else {
                    match c {
                        '.' => format!("\x1b[38;2;0;0;0m{c}{c}\x1b[38;2;255;255;255m"),
                        '#' => format!("\x1b[38;2;0;0;255m{c}{c}\x1b[38;2;255;255;255m"),
                        '*' => format!("\x1b[38;2;128;128;255m{c}{c}\x1b[38;2;255;255;255m"),
                        _ => format!("{c}{c}"),
                    }
                };
                s.push_str(&str);
            }
            s.push('\n');
        }
        s.push_str("\n\n\n\n\n\n\n\n\n\n");
        s
    }
    fn step(&mut self, mut rs: Vrd) -> Vrd {
        if rs.is_empty() {
            let r = Reindeer::new(&self.s);
            let d = r.p == self.e;
            rs.push_back((r, d));
        }
        let mut stepped_rs = VecDeque::new();
        let mut cheapest = usize::MAX;
        // let min_cost = if let Some(c) = rs.iter().min_by_key(|v| if !v.1 { v.0.cost() } else {usize::MAX}).iter().next() {
        //     c.0.cost()
        // } else { usize::MAX };

        // print!("{}", self.map_str_w_r_cost(&rs));
        // println!(" ");
        while let Some((r, is_done)) = rs.pop_front() {
            if is_done {
                cheapest = r.cost();
                stepped_rs.push_back((r, true));
                continue;
            }
            // if r.cost() > min_cost {
            //     stepped_rs.push_back((r, is_done));
            //     continue;
            // }
            let last_d = r.d.clone();
            for d in Dir::RDLU {
                let mut r = r.clone();
                r.d = d;
                if r.d != last_d {
                    r.t += 1;
                    if r.d.is_180(&last_d) {
                        r.t += 1;
                    }
                }
                r.p.x = r.p.x.wrapping_add(r.d.dx);
                r.p.y = r.p.y.wrapping_add(r.d.dy);
                r.s += 1;
                r.path.push(r.p.clone());
                let s = &mut self.m[r.p.y][r.p.x];
                let rc = r.cost();
                if rc <= cheapest && rc <= s.c {
                    s.c = rc + 1000;
                    let is_done = match s.k {
                        Kind::Empty => false,
                        Kind::End => true,
                        Kind::Wall | Kind::Start => panic!("can't have cost less than 0"),
                    };
                    if is_done {
                        r.dur = r.start.elapsed();
                    }
                    stepped_rs.push_back((r, is_done));
                }
            }
        }
        stepped_rs
    }
    fn step_all(&mut self, mut rs: Vrd, d_ms: u64) -> Vrd {
        loop {
            if !rs.is_empty() && rs.iter().all(|(_, is_done)| *is_done) {
                return rs;
            } else {
                rs = self.step(rs);
                if d_ms != 0 {
                    print!("{}", self.colored_map_str_w_r(&rs));
                    // print!("\x1B[2J\x1B[H{}", self.colored_map_str_w_r(&rs));
                    std::thread::sleep(Duration::from_millis(d_ms));
                }
            }
        }
    }
    fn step_all_cheapest_one(&mut self, rs: Vrd, d_ms: u64) -> Reindeer {
        let v = self.step_all(rs, d_ms);
        v.into_iter().min_by_key(|r| r.0.cost()).unwrap().0
    }
    fn best_seats(&self, rs: &Vrd) -> usize {
        rs.iter().min_set_by_key(|(r, _)| r.cost()).iter().flat_map(|(r, _)| r.path.clone()).unique().count()
    }
    fn best_seats_map(&self, rs: &Vrd) -> String {
        let mut m = self.m.iter().map(|r|{
            r.iter().map(Space::to_char).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        for p in rs.iter().min_set_by_key(|(r, _)| r.cost()).iter().flat_map(|(r, _)| r.path.clone()).unique() {
            m[p.y][p.x] = 'O'
        }
        String::from_iter(m.into_iter().flat_map(|r| {
            r.into_iter().chain(once('\n'))
        }))
    }
}
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.map_str())
    }
}
fn main() {
    let _ = *START;
    println!("START {:?}", START.elapsed());
    // let mut m = Map::new(d());
    // let rs = m.step_all(VecDeque::new(), 0);
    // let mut min_cost = usize::MAX;
    // for (r, b) in rs.iter().sorted_by_key(|(r, _)| r.cost()) {
    //     if r.cost() <= min_cost {
    //         min_cost = r.cost();
    //         println!("{b} {r:#?} {}", r.cost());
    //     }
    // }
    let mut m = Map::new(d());
    let rs = m.step_all(VecDeque::new(), 0);
    let c = m.best_seats(&rs);
    println!("{c}");
    println!("END {:?}", START.elapsed());
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t_p1_e1_map_str() {
        assert_eq!(e1(), Map::new(e1()).map_str());
    }
    #[test]
    fn t_p1_e1_cheapest() {
        assert_eq!(7036, Map::new(e1()).step_all_cheapest_one(VecDeque::new(), 0).cost());
    }
    #[test]
    fn t_p1_e2_cheapest() {
        assert_eq!(11048, Map::new(e2()).step_all_cheapest_one(VecDeque::new(), 0).cost());
    }
    #[test]
    fn t_p1_d_cheapest() {
        assert_eq!(108504, Map::new(d()).step_all_cheapest_one(VecDeque::new(), 0).cost());
    }

    fn best_seats(d: &str) -> usize {
        let mut m = Map::new(d);
        let rs = m.step_all(VecDeque::new(), 0);
        m.best_seats(&rs)
    }
    #[test]
    fn t_p2_e1_best_seats() {
        assert_eq!(45, best_seats(e1()));
    }
    #[test]
    fn t_p2_e2_best_seats() {
        assert_eq!(64, best_seats(e2()));
    }
    #[test]
    fn t_p2_d_best_seats() {
        assert_eq!(538, best_seats(d()));
    }
}
