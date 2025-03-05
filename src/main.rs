#![allow(dead_code)]

use std::cell::RefCell;
use std::iter::repeat;
use std::rc::Rc;
use std::rc::Weak;

use itertools::Itertools;

fn e1() -> &'static str {
    "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"
}
fn e1_p1_fastest() -> usize {
    84
}
fn d() -> &'static str {
    include_str!("input.txt")
}

const CHEAT_SIZE: usize = 2;
struct Space {
    kind: char,
    x: usize,
    y: usize,
    up: Weak<RefCell<Self>>,
    right: Weak<RefCell<Self>>,
    down: Weak<RefCell<Self>>,
    left: Weak<RefCell<Self>>,
}
impl Space {
    fn new(
        kind: char,
        x: usize,
        y: usize,
        right: Weak<RefCell<Self>>,
        down: Weak<RefCell<Self>>,
        left: Weak<RefCell<Self>>,
        up: Weak<RefCell<Self>>,
    ) -> Self {
        Self {
            kind,
            x,
            y,
            right,
            down,
            left,
            up,
        }
    }
}
struct Map {
    spaces: Vec<Vec<Rc<RefCell<Space>>>>,
    width: usize,
    height: usize,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}
impl Map {
    fn new(d: &str) -> Self {
        let mut spaces = Vec::new();
        for l in d.lines() {
            let mut r = Vec::new();
            for c in l.chars() {
                r.push(Rc::new(RefCell::new(Space::new(
                    c,
                    r.len(),
                    spaces.len(),
                    Weak::new(),
                    Weak::new(),
                    Weak::new(),
                    Weak::new(),
                ))));
            }
            spaces.push(r);
        }
        assert!(spaces.iter().is_sorted_by(|a, b| a.len() == b.len()));
        let (width, height) = (spaces[0].len(), spaces.len());
        for d in [width, height] {
            assert_ne!(d, 0);
        }
        let (mut start_x, mut start_y, mut end_x, mut end_y) =
            repeat(usize::MAX).next_tuple().unwrap();
        for y in 0..height {
            for x in 0..width {
                let mut s = spaces[y][x].borrow_mut();
                if s.kind == 'S' {
                    start_x = x;
                    start_y = y;
                } else if s.kind == 'E' {
                    end_x = x;
                    end_y = y;
                }
                if x < width - 1 {
                    s.right = Rc::downgrade(&spaces[y][x + 1]);
                }
                if y < height - 1 {
                    s.down = Rc::downgrade(&spaces[y + 1][x]);
                }
                if x > 0 {
                    s.left = Rc::downgrade(&spaces[y][x - 1]);
                }
                if y > 0 {
                    s.up = Rc::downgrade(&spaces[y - 1][x]);
                }
            }
        }
        for p in [start_x, start_y, end_x, end_y] {
            assert_ne!(p, usize::MAX);
        }
        Self {
            spaces,
            width,
            height,
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn e1_nav_start1() {
        let m = Map::new(e1());
        let s = &m.spaces[0][0];
        let s = s.borrow().down.upgrade().unwrap();
        let s = s.borrow().down.upgrade().unwrap();
        let s = s.borrow().down.upgrade().unwrap();
        let s = s.borrow().down.upgrade().unwrap();
        let s = s.borrow().right.upgrade().unwrap();
        let s = s.borrow().up.upgrade().unwrap();
        let s = s.borrow();
        assert_eq!((s.kind, s.x, s.y), ('S', 1, 3));
    }
    #[test]
    fn e1_nav_start2() {
        let m = Map::new(e1());
        let s = &m.spaces[5][1];
        let s = s.borrow().up.upgrade().unwrap();
        let s = s.borrow().up.upgrade().unwrap();
        let s = s.borrow();
        assert_eq!((s.kind, s.x, s.y), ('S', 1, 3));
    }
    #[test]
    fn e1_nav_end1() {
        let m = Map::new(e1());
        let s = &m.spaces[0][0];
        let s = s.borrow().down.upgrade().unwrap();
        let s = s.borrow().down.upgrade().unwrap();
        let s = s.borrow().down.upgrade().unwrap();
        let s = s.borrow().down.upgrade().unwrap();
        let s = s.borrow().down.upgrade().unwrap();
        let s = s.borrow().down.upgrade().unwrap();
        let s = s.borrow().down.upgrade().unwrap();
        let s = s.borrow().right.upgrade().unwrap();
        let s = s.borrow().right.upgrade().unwrap();
        let s = s.borrow().right.upgrade().unwrap();
        let s = s.borrow().right.upgrade().unwrap();
        let s = s.borrow().right.upgrade().unwrap();
        let s = s.borrow();
        assert_eq!((s.kind, s.x, s.y), ('E', 5, 7));
    }
    #[test]
    fn e1_nav_end2() {
        let m = Map::new(e1());
        let s = &m.spaces[8][6];
        let s = s.borrow().up.upgrade().unwrap();
        let s = s.borrow().left.upgrade().unwrap();
        let s = s.borrow();
        assert_eq!((s.kind, s.x, s.y), ('E', 5, 7));
    }
    #[test]
    fn e1_nav_end3() {
        let m = Map::new(e1());
        let s = &m.spaces[m.end_y + 1][m.end_x + 1];
        let s = s.borrow().up.upgrade().unwrap();
        let s = s.borrow().left.upgrade().unwrap();
        let s = s.borrow();
        assert_eq!((s.kind, s.x, s.y), ('E', 5, 7));
    }
    #[test]
    fn e1_nav_oob_up() {
        let m = Map::new(e1());
        let s = &m.spaces[0][0];
        let s = s.borrow().up.upgrade();
        assert!(s.is_none());
    }
    #[test]
    fn e1_nav_oob_right() {
        let m = Map::new(e1());
        let s = &m.spaces[0][14];
        let s = s.borrow().right.upgrade();
        assert!(s.is_none());
    }
}
