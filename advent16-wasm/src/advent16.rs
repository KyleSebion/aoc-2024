use std::collections::VecDeque;
use itertools::Itertools;
pub fn d() -> &'static str {
    include_str!("input.txt")
}
pub enum Kind {
    Empty,
    Wall,
    Start,
    End,
}
pub struct Space {
    pub k: Kind,
    pub c: usize,
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
}
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
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
pub struct Reindeer {
    pub p: Pos,
    d: Dir,
    t: usize,
    s: usize,
    path: Vec<Pos>,
}
impl Reindeer {
    fn new(p: &Pos) -> Self {
        let mut s =        Self {
            p: Pos { x: p.x, y: p.y },
            d: Dir::RIGHT,
            t: 0,
            s: 0,
            path: Vec::new(),
        };
        s.path.push(s.p.clone());
        s
    }
    fn cost(&self) -> usize {
        self.t * 1000 + self.s
    }
}
pub type Vrd = VecDeque<(Reindeer, bool)>;
pub struct Map {
    pub m: Vec<Vec<Space>>,
    pub s: Pos,
    pub e: Pos,
}
impl Map {
    pub fn new(d: &str) -> Self {
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
    fn step(&mut self, mut rs: Vrd) -> Vrd {
        if rs.is_empty() {
            let r = Reindeer::new(&self.s);
            let d = r.p == self.e;
            rs.push_back((r, d));
        }
        let mut stepped_rs = VecDeque::new();
        let mut cheapest = usize::MAX;
        while let Some((r, is_done)) = rs.pop_front() {
            if is_done {
                cheapest = r.cost();
                stepped_rs.push_back((r, true));
                continue;
            }
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
                    stepped_rs.push_back((r, is_done));
                }
            }
        }
        stepped_rs
    }
    pub fn step_once(&mut self, rs: Vrd) -> Result<Vrd, Vrd> {
        if !rs.is_empty() && rs.iter().all(|(_, is_done)| *is_done) {
            Ok(rs)
        } else {
            Err(self.step(rs))
        }
    }
    pub fn best_seats(&self, rs: &Vrd) -> usize {
        rs.iter().min_set_by_key(|(r, _)| r.cost()).iter().flat_map(|(r, _)| r.path.clone()).unique().count()
    }
}
