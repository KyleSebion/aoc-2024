#![allow(dead_code)]
use std::{collections::VecDeque, iter};
fn e1() -> &'static str {
    "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"
}
fn e1_multi_map() -> &'static str {
    "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########
########
#....OO#
##.@...#
#...O..#
#.#.O..#
#...O..#
#...O..#
########
########
#....OO#
##.....#
#..@O..#
#.#.O..#
#...O..#
#...O..#
########
########
#....OO#
##.....#
#...@O.#
#.#.O..#
#...O..#
#...O..#
########
########
#....OO#
##.....#
#....@O#
#.#.O..#
#...O..#
#...O..#
########
########
#....OO#
##.....#
#.....O#
#.#.O@.#
#...O..#
#...O..#
########
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
"
}
fn e1_sum() -> usize { 2028 }
fn e2() -> &'static str {
    "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
}
fn e2_end_map() -> &'static str {
    "\
##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
"
}
fn e2_sum() -> usize { 10092 }
fn d() -> &'static str {
    include_str!("input.txt")
}
fn d_sum() -> usize { 1436690 }
#[derive(Clone)]
enum Kind {
    Empty,
    Wall,
    Box,
    Robot,
}
#[derive(Clone)]
struct Space {
    k: Kind,
}
impl Space {
    fn new(k: char) -> Self {
        let k = match k {
            '.' => Kind::Empty,
            '#' => Kind::Wall,
            'O' => Kind::Box,
            '@' => Kind::Robot,
            _ => panic!("wrong kind")
        };
        Self { k }
    }
    fn get_char(&self) -> char {
        match self.k {
            Kind::Empty => '.',
            Kind::Wall => '#',
            Kind::Box => 'O',
            Kind::Robot => '@',
        }
    }
}
impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.get_char())
    }
}
struct Dir {
    dx: usize,
    dy: usize,
}
impl Dir {
    fn new(d: char) -> Self {
        let (dx, dy) = match d {
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (usize::MAX, 0),
            '^' => (0, usize::MAX),
            _ => panic!("wrong dir")
        };
        Self {dx, dy }
    }
    fn get_char(&self) -> char {
        match (self.dx, self.dy) {
            (1, 0) => '>',
            (0, 1) => 'v',
            (usize::MAX, 0) => '<',
            (0, usize::MAX) => '^',
            _ => panic!("wrong dir")
        }
    }
}
impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.get_char())
    }
}
#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}
struct Map {
    m: Vec<Vec<Space>>,
    d: VecDeque<Dir>,
    p: Pos,
}
impl Map {
    fn new(d: &str) -> Self {
        let mut m = Self {
            m: Vec::new(),
            d: VecDeque::new(),
            p: Pos { x: 0, y: 0, },
        };
        let mut l = d.lines();
        for l in l.by_ref() {
            if l.is_empty() { break; }
            let mut r = Vec::new();
            for (i, c) in l.char_indices() {
                let s = Space::new(c);
                if let Kind::Robot = s.k {
                    m.p.x = i;
                    m.p.y = m.m.len();
                }
                r.push(s);
            }
            m.m.push(r);
        }
        assert!(m.m.iter().is_sorted_by(|a, b| a.len() == b.len()));
        for l in l.by_ref() {
            for c in l.chars() {
                m.d.push_back(Dir::new(c));
            }
        }
        m
    }
    fn map_str(&self) -> String {
        let mut s = String::new();
        for r in self.m.iter() {
            for c in r.iter().map(Space::get_char) {
                s.push(c);
            }
            s.push('\n');
        }
        s
    }
    fn map_str2(&self) -> String {
        let mut s = String::new();
        for r in self.m.iter() {
            for c in r.iter().map(Space::get_char) {
                s.push(' ');
                s.push(c);
            }
            s.push('\n');
        }
        s
    }
    fn dir_str(&self) -> String {
        String::from_iter(self.d.iter().map(Dir::to_string))
    }
    fn pos_str(&self) -> String {
        format!("{:?}", self.p)
    }
    fn move_to_next(&mut self, x: usize, y: usize, nx: usize, ny: usize) -> bool {
        let s = self.m[y][x].clone();
        if let Kind::Robot = s.k {
            self.p.x = nx;
            self.p.y = ny;
        }
        self.m[ny][nx] = s;
        self.m[y][x] = Space { k: Kind::Empty };
        true
    }
    fn try_step(&mut self, d: &Dir, x: usize, y: usize) -> bool {
        let nx = x.wrapping_add(d.dx);
        let ny = y.wrapping_add(d.dy);
        match self.m[ny][nx].k {
            Kind::Robot => panic!("multiple robots?"),
            Kind::Wall => false,
            Kind::Empty => self.move_to_next(x, y, nx, ny),
            Kind::Box => {
                if self.try_step(d, nx, ny) {
                    self.move_to_next(x, y, nx, ny)
                } else {
                    false
                }
            }
        }
    }
    fn step(&mut self) -> Option<()> {
        let d = self.d.pop_front()?;
        self.try_step(&d, self.p.x, self.p.y);
        Some(())
    }
    fn walk(&mut self) {
        iter::from_fn(|| self.step()).last();
    }
    fn box_sum(&self) -> usize {
        self.m.iter().enumerate().fold(0, |a, (y, r)| {
            a + r.iter().enumerate().fold(0, |a, (x, s)| {
                a + if let Kind::Box = s.k {
                    y * 100 + x
                } else {
                    0
                }
            })
        })
    }
}
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}\n{}\n{}", self.pos_str(), self.map_str(), self.dir_str())
    }
}
fn dp1_animated() {
    let mut m = Map::new(d());
    while m.step().is_some() {
        println!("\x1B[2J\x1B[H{}", m.map_str2());
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    println!("{}", m.box_sum());
}


fn e3() -> &'static str {
    "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
"
}
fn e3_map1() -> &'static str {
    "\
##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############

<vv<<^^<<^^
"
}
fn e3_multi_map() -> &'static str {
    "\
##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############
##############
##......##..##
##..........##
##...[][]@..##
##....[]....##
##..........##
##############
##############
##......##..##
##..........##
##...[][]...##
##....[].@..##
##..........##
##############
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.......@..##
##############
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##......@...##
##############
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.....@....##
##############
##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############
##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############
##############
##......##..##
##...[][]...##
##....[]....##
##....@.....##
##..........##
##############
##############
##......##..##
##...[][]...##
##....[]....##
##...@......##
##..........##
##############
##############
##......##..##
##...[][]...##
##...@[]....##
##..........##
##..........##
##############
##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############
"
}
fn e2_p2_end_map() -> &'static str {
    "\
####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################
"
}
fn e2_p2_sum() -> usize { 9021 }
fn d_p2_sum() -> usize { 1482350 }
fn d_p2_end_map() -> &'static str {
    include_str!("endmap.txt")
}
#[derive(Clone)]
enum Kind2 {
    Empty,
    Wall,
    BoxL,
    BoxR,
    Robot,
}
#[derive(Clone)]
struct Space2 {
    k: Kind2,
}
impl Space2 {
    fn new(k: char) -> Vec<Self> {
        let k = match k {
            '.' => vec![Kind2::Empty, Kind2::Empty],
            '#' => vec![Kind2::Wall, Kind2::Wall],
            'O' => vec![Kind2::BoxL, Kind2::BoxR],
            '@' => vec![Kind2::Robot, Kind2::Empty],
            _ => panic!("wrong kind")
        };
        k.into_iter().map(|k| Self { k }).collect()
    }
    fn get_char(&self) -> char {
        match self.k {
            Kind2::Empty => '.',
            Kind2::Wall => '#',
            Kind2::BoxL => '[',
            Kind2::BoxR => ']',
            Kind2::Robot => '@',
        }
    }
}
impl std::fmt::Display for Space2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.get_char())
    }
}
struct Map2 {
    m: Vec<Vec<Space2>>,
    d: VecDeque<Dir>,
    p: Pos,
}
impl Map2 {
    fn new(d: &str) -> Self {
        let mut m = Self {
            m: Vec::new(),
            d: VecDeque::new(),
            p: Pos { x: 0, y: 0, },
        };
        let mut l = d.lines();
        for l in l.by_ref() {
            if l.is_empty() { break; }
            let mut r = Vec::new();
            for (i, c) in l.char_indices() {
                let mut s = Space2::new(c);
                if let Kind2::Robot = s[0].k {
                    m.p.x = i * 2;
                    m.p.y = m.m.len();
                }
                r.append(&mut s);
            }
            m.m.push(r);
        }
        assert!(m.m.iter().is_sorted_by(|a, b| a.len() == b.len()));
        for l in l.by_ref() {
            for c in l.chars() {
                m.d.push_back(Dir::new(c));
            }
        }
        m
    }
    fn map_str(&self) -> String {
        let mut s = String::new();
        for r in self.m.iter() {
            for c in r.iter().map(Space2::get_char) {
                s.push(c);
            }
            s.push('\n');
        }
        s
    }
    fn dir_str(&self) -> String {
        String::from_iter(self.d.iter().map(Dir::to_string))
    }
    fn pos_str(&self) -> String {
        format!("{:?}", self.p)
    }
    fn move_to_next(&mut self, x: usize, y: usize, nx: usize, ny: usize) {
        let s = self.m[y][x].clone();
        if let Kind2::Robot = s.k {
            self.p.x = nx;
            self.p.y = ny;
        }
        self.m[ny][nx] = s;
        self.m[y][x] = Space2 { k: Kind2::Empty };
    }
    fn do_move(&mut self, d: &Dir, x: usize, y: usize, one_side: bool) {
        let nx = x.wrapping_add(d.dx);
        let ny = y.wrapping_add(d.dy);
        match self.m[y][x].k {
            Kind2::Robot => {
                self.do_move(d, nx, ny, false);
                self.move_to_next(x, y, nx, ny);
            }
            Kind2::Wall => {}
            Kind2::Empty => {}
            Kind2::BoxL | Kind2::BoxR => {
                if d.dy == 0 || one_side {
                    self.do_move(d, nx, ny, false);
                    self.move_to_next(x, y, nx, ny);
                } else {
                    let ox = x.wrapping_add(if let Kind2::BoxL = self.m[y][x].k { 1 } else { usize::MAX });
                    self.do_move(d, x, y, true);
                    self.do_move(d, ox, y, true);
                }
            }
        }
    }
    fn try_move(&mut self, d: &Dir, x: usize, y: usize) {
        if self.can_move(d, x, y, false) {
            self.do_move(d, x, y, false);
        }
    }
    fn can_move(&mut self, d: &Dir, x: usize, y: usize, one_side: bool) -> bool {
        let nx = x.wrapping_add(d.dx);
        let ny = y.wrapping_add(d.dy);
        match self.m[y][x].k {
            Kind2::Robot => {
                self.can_move(d, nx, ny, false)
            }
            Kind2::Wall => false,
            Kind2::Empty => true,
            Kind2::BoxL | Kind2::BoxR => {
                if d.dy == 0 || one_side {
                    self.can_move(d, nx, ny, false)
                } else {
                    let ox = x.wrapping_add(if let Kind2::BoxL = self.m[y][x].k { 1 } else { usize::MAX });
                    self.can_move(d, x, y, true) && 
                    self.can_move(d, ox, y, true)
                }
            }
        }
    }
    fn step(&mut self) -> Option<()> {
        let d = self.d.pop_front()?;
        self.try_move(&d, self.p.x, self.p.y);
        Some(())
    }
    fn walk(&mut self) {
        iter::from_fn(|| self.step()).last();
    }
    fn box_sum(&self) -> usize {
        self.m.iter().enumerate().fold(0, |a, (y, r)| {
            a + r.iter().enumerate().fold(0, |a, (x, s)| {
                a + if let Kind2::BoxL = s.k {
                    y * 100 + x
                } else {
                    0
                }
            })
        })
    }
}
impl std::fmt::Display for Map2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}\n{}\n{}", self.pos_str(), self.map_str(), self.dir_str())
    }
}
fn dp2_animated() {
    let mut m = Map2::new(d());
    while m.step().is_some() {
        println!("\x1B[2J\x1B[H{}", m.map_str());
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    println!("{}", m.box_sum());
}
fn main() {
    dp2_animated();
}
#[cfg(test)]
mod test {
    use super::*;
    fn walked_map(d: &str) -> Map {
        let mut m = Map::new(d);
        m.walk();
        m
    }
    fn walked_sum(d: &str) -> usize {
        walked_map(d).box_sum()
    }
    #[test] fn e1_map() {
        let mut m = Map::new(e1());
        let mut s = String::new();
        s.push_str(&m.map_str());
        while m.step().is_some() {
            s.push_str(&m.map_str());
        }
        assert_eq!(e1_multi_map(), &s);
    }
    #[test] fn e2_map() {
        let mut m = Map::new(e2());
        m.walk();
        assert_eq!(e2_end_map(), m.map_str());
    }
    #[test] fn e1_s() { assert_eq!(e1_sum(), walked_sum(e1())) }
    #[test] fn e2_s() { assert_eq!(e2_sum(), walked_sum(e2())) }
    #[test] fn d_s() { assert_eq!(d_sum(), walked_sum(d())) }

    fn e3_map() -> Map2 {
        Map2::new(e3())
    }
    #[test] fn e3_map1_test() { let m = e3_map(); assert_eq!(format!("{}\n{}\n", m.map_str(), m.dir_str()), e3_map1()) }
    #[test] fn e3_multi_map_test() {
        let mut m = e3_map();
        let mut s = String::new();
        s.push_str(&m.map_str());
        while m.step().is_some() {
            s.push_str(&m.map_str());
        }
        assert_eq!(e3_multi_map(), &s);
    }
    #[test] fn e2_p2_end_map_test() {
        let mut m = Map2::new(e2());
        m.walk();
        assert_eq!(e2_p2_end_map(), m.map_str());
    }
    #[test] fn e2_p2_sum_test() {
        let mut m = Map2::new(e2());
        m.walk();
        assert_eq!(e2_p2_sum(), m.box_sum());
    }
    #[test] fn d_p2_sum_test() {
        let mut m = Map2::new(d());
        m.walk();
        assert_eq!(d_p2_sum(), m.box_sum());
    }
    #[test] fn d_p2_end_map_test() {
        let mut m = Map2::new(d());
        m.walk();
        assert_eq!(d_p2_end_map(), m.map_str());
    }
}