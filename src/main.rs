#![allow(dead_code, clippy::unit_cmp)]

use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;
use std::rc::Weak;
use std::thread;
use std::time::Instant;
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
struct SpaceInner {
    kind: char,
    x: usize,
    y: usize,
    up: Weak<Space>,
    right: Weak<Space>,
    down: Weak<Space>,
    left: Weak<Space>,
    min_steps_to: usize,
}
struct Space(RefCell<SpaceInner>);
impl Space {
    fn new(kind: char) -> Rc<Self> {
        Rc::new(Self(RefCell::new(SpaceInner {
            kind,
            x: 0,
            y: 0,
            right: Weak::new(),
            down: Weak::new(),
            left: Weak::new(),
            up: Weak::new(),
            min_steps_to: usize::MAX,
        })))
    }
    fn borrow(self: &Rc<Self>) -> Ref<'_, SpaceInner> {
        self.0.borrow()
    }
    fn borrow_mut(self: &Rc<Self>) -> RefMut<'_, SpaceInner> {
        self.0.borrow_mut()
    }
    fn cant_step_to(self: &Rc<Self>, p: char) -> bool {
        self.get_k() == '#' || (self.get_k() == '2' && p != '1')
    }
    fn at_start(self: &Rc<Self>) -> bool {
        self.get_k() == 'S'
    }
    fn at_end(self: &Rc<Self>) -> bool {
        self.get_k() == 'E'
    }
    fn get_w(self: &Rc<Self>) -> Weak<Self> {
        Rc::downgrade(self)
    }
    fn get_r(self: &Rc<Self>) -> Weak<Self> {
        self.borrow().right.clone()
    }
    fn get_d(self: &Rc<Self>) -> Weak<Self> {
        self.borrow().down.clone()
    }
    fn get_l(self: &Rc<Self>) -> Weak<Self> {
        self.borrow().left.clone()
    }
    fn get_u(self: &Rc<Self>) -> Weak<Self> {
        self.borrow().up.clone()
    }
    fn get_x(self: &Rc<Self>) -> usize {
        self.borrow().x
    }
    fn get_y(self: &Rc<Self>) -> usize {
        self.borrow().y
    }
    fn get_k(self: &Rc<Self>) -> char {
        self.borrow().kind
    }
    fn set_k(self: &Rc<Self>, k: char) {
        self.borrow_mut().kind = k;
    }
    fn get_min_steps_to(self: &Rc<Self>) -> usize {
        self.borrow().min_steps_to
    }
    fn reset_min_steps(self: &Rc<Self>) {
        self.borrow_mut().min_steps_to = usize::MAX;
    }
    fn set_min_steps(self: &Rc<Self>, steps: usize) {
        self.borrow_mut().min_steps_to = steps;
    }
    fn step(self: &Rc<Self>, cur_steps: usize, pre: char, dp: bool, di: bool) -> Option<usize> {
        let np = !dp;
        let ind = if di { cur_steps * 2 } else { 0 };
        let p = format!("{:ind$}{},{}", "", self.get_x(), self.get_y());
        if self.get_min_steps_to() < cur_steps {
            let _ = np || () == println!("{p} {} < {}", self.get_min_steps_to(), cur_steps);
            return None;
        }
        if self.cant_step_to(pre) {
            let _ = np || () == println!("{p} cant_step_to");
            return None;
        }
        let _ = np || () == println!("{p} min_steps_to = {cur_steps}");
        self.set_min_steps(cur_steps);
        if self.at_end() {
            let _ = np || () == println!("{p} at_end = {cur_steps}");
            return Some(cur_steps);
        }
        let mut costs = Vec::new();
        let sb = self.borrow();
        for w in [&sb.right, &sb.down, &sb.left, &sb.up] {
            if let Some(s) = w.upgrade() {
                let _ = np
                    || () == println!("{p} trying {},{} {}", s.get_x(), s.get_y(), cur_steps + 1);
                if let Some(steps) = s.step(cur_steps + 1, self.get_k(), dp, di) {
                    costs.push(steps);
                }
            }
        }
        let ret = costs.into_iter().min();
        let _ = np || () == println!("{p} ret cur_steps+1 == {} {:?} ", cur_steps + 1, ret);
        ret
    }
}
#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}
struct Map {
    spaces: Vec<Vec<Rc<Space>>>,
    width: usize,
    height: usize,
    start: Weak<Space>,
    end: Weak<Space>,
}
impl Map {
    fn new(d: &str) -> Self {
        let mut spaces = Vec::new();
        for l in d.lines() {
            let mut r = Vec::new();
            for c in l.chars() {
                r.push(Space::new(c));
            }
            spaces.push(r);
        }
        assert!(spaces.iter().is_sorted_by(|a, b| a.len() == b.len()));
        let (width, height) = (spaces[0].len(), spaces.len());
        for d in [width, height] {
            assert_ne!(d, 0);
        }
        let (mut start, mut end) = (Weak::new(), Weak::new());
        for y in 0..height {
            for x in 0..width {
                let space = &spaces[y][x];
                let mut s = space.borrow_mut();
                s.x = x;
                s.y = y;
                if s.kind == 'S' {
                    start = Rc::downgrade(space);
                } else if s.kind == 'E' {
                    end = Rc::downgrade(space);
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
        for p in [&start, &end] {
            assert!(p.upgrade().is_some());
        }
        Self {
            spaces,
            width,
            height,
            start,
            end,
        }
    }
    fn get_steps_s_to_e(&self, do_print: bool, do_ind: bool) -> Option<usize> {
        let ret = self.start.upgrade()?.step(0, ' ', do_print, do_ind);
        for r in &self.spaces {
            for s in r {
                s.reset_min_steps();
            }
        }
        ret
    }
    fn get_map_string(&self) -> String {
        self.spaces
            .iter()
            .map(|r| r.iter().map(Space::get_k).join(""))
            .join("\n")
    }
    fn panic_oob_pos(&self, c: &Pos) {
        if c.x >= self.width || c.y >= self.height {
            panic!("{c:?} oob")
        }
    }
    fn get_steps_s_to_e_cheat_pos(&self, c1: Pos, c2: Pos) -> Option<usize> {
        let mut old = Vec::new();
        for (p, c) in [(c1, '1'), (c2, '2')] {
            self.panic_oob_pos(&p);
            let k = self.spaces[p.y][p.x].get_k();
            self.spaces[p.y][p.x].set_k(c);
            old.push((p, k));
        }
        println!("{}", self.get_map_string());
        let s = self.get_steps_s_to_e(false, false);
        old.into_iter().for_each(|(p, c)| self.spaces[p.y][p.x].set_k(c));
        s
    }
    fn get_steps_s_to_e_cheat(&self, c1: Weak<Space>, c2: Weak<Space>) -> Option<usize> {
        let mut old = Vec::new();
        for (s, c) in [(c1, '1'), (c2, '2')] {
            let s = s.upgrade()?;
            let k = s.get_k();
            s.set_k(c);
            old.push((s, k));
        }
        let s = self.get_steps_s_to_e(false, false);
        old.into_iter().for_each(|(s, c)| s.set_k(c));
        s
    }
}
fn run1() {
    let s = Instant::now();
    let m = Map::new(e1());
    let h = m.height;
    let w = m.width;
    let c = h * w;
    // let steps = m.get_steps_s_to_e(true, false); // 5s (in release mode)
    // let steps = m.get_steps_s_to_e(false, false); // 11ms (in release mode)

    // let steps = m.get_steps_s_to_e_cheat_pos(Pos { x: 8, y: 1 }, Pos { x: 9, y: 1 }); //72
    // let steps = m.get_steps_s_to_e_cheat_pos(Pos { x: 10, y: 7 }, Pos { x: 11, y: 7 }); //64

    let steps = m.get_steps_s_to_e_cheat(m.spaces[1][8].get_w(), m.spaces[1][9].get_w()); //72
    // let steps = m.get_steps_s_to_e_cheat(m.spaces[7][10].get_w(), m.spaces[7][11].get_w()); //64
    println!("{w}x{h} {c} {:?} {:?}", steps, s.elapsed());
}
fn run_with_big_stack_and_wait(f: fn()) {
    thread::Builder::new()
        .stack_size(1024 * 1024 * 1024)
        .spawn(f)
        .unwrap()
        .join()
        .unwrap();
}
fn run_with_big_stack_and_wait_and_ret<T: Send + 'static>(f: fn() -> T) -> T {
    thread::Builder::new()
        .stack_size(1024 * 1024 * 1024)
        .spawn(f)
        .unwrap()
        .join()
        .unwrap()
}
fn main() {
    run_with_big_stack_and_wait(run1);
}
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
    #[test]
    fn e1_s_to_e() {
        let m = Map::new(e1());
        assert_eq!(Some(84), m.get_steps_s_to_e(false, false));
    }
    #[test]
    fn d_s_to_e() {
        let res = run_with_big_stack_and_wait_and_ret(|| {
            let m = Map::new(d());
            m.get_steps_s_to_e(false, false)
        });
        assert_eq!(Some(9348), res);
    }
}
