#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

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
        up: Weak<RefCell<Self>>,
        right: Weak<RefCell<Self>>,
        down: Weak<RefCell<Self>>,
        left: Weak<RefCell<Self>>,
    ) -> Self {
        Self {
            kind,
            x,
            y,
            up,
            right,
            down,
            left,
        }
    }
}
struct Map {
    spaces: Vec<Vec<Rc<RefCell<Space>>>>,
    width: usize,
    height: usize,
}
impl Map {
    fn make_weak_space(
        spaces: &[Vec<Rc<RefCell<Space>>>],
        x: usize,
        y: usize,
    ) -> Weak<RefCell<Space>> {
        Rc::downgrade(&Rc::clone(&spaces[y][x]))
    }
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
        let (width, height) = (spaces[0].len(), spaces.len());
        assert!(spaces.iter().is_sorted_by(|a, b| a.len() == b.len()));
        assert!(width > 0);
        assert!(height > 0);
        for y in 0..height {
            for x in 0..width {
                let mut s = spaces[y][x].borrow_mut();
                if y > 0 {
                    s.up = Self::make_weak_space(&spaces, x, y - 1);
                }
                if x > 0 {
                    s.left = Self::make_weak_space(&spaces, x - 1, y);
                }
                if y < height - 1 {
                    s.down = Self::make_weak_space(&spaces, x, y + 1);
                }
                if x < width - 1 {
                    s.right = Self::make_weak_space(&spaces, x + 1, y);
                }
            }
        }
        Self {
            spaces,
            width,
            height,
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
}
