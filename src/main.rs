#![allow(dead_code, clippy::unit_cmp)]

use itertools::Itertools;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;
use std::thread;
use std::time::Instant;

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
    cheat: char,
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
            cheat: ' ',
        })))
    }
    fn borrow(self: &Rc<Self>) -> Ref<'_, SpaceInner> {
        self.0.borrow()
    }
    fn borrow_mut(self: &Rc<Self>) -> RefMut<'_, SpaceInner> {
        self.0.borrow_mut()
    }
    fn can_step_to(self: &Rc<Self>, pre_c: char) -> bool {
        let c = self.get_c();
        let k = self.get_k();
        match (k == '#', c == '1', c == '2', pre_c == '1') {
            // nothing special; not wall
            (false, false, false, false) => true,

            // nothing special; wall
            (true, false, false, false) => false,

            // c 1
            (false, true, false, false) => true,
            (true, true, false, false) => true,

            // c 2 and pre 1 and c 2 not wall
            (false, false, true, true) => true,

            // c 2 and pre 1 and c 2 wall
            (true, false, true, true) => false,

            // c 2 and pre not 1
            (false, false, true, false) => false,
            (true, false, true, false) => false,

            // c not 2 and pre 1
            (false, false, false, true) => false,
            (true, false, false, true) => false,

            //cannot happen
            (false, true, false, true) => panic!("no"),
            (true, true, false, true) => panic!("no"),
            (false, true, true, false) => panic!("no"),
            (true, true, true, false) => panic!("no"),
            (false, true, true, true) => panic!("no"),
            (true, true, true, true) => panic!("no"),
        }
    }
    fn cant_step_to(self: &Rc<Self>, pre_c: char) -> bool {
        !self.can_step_to(pre_c)
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
    fn get_c(self: &Rc<Self>) -> char {
        self.borrow().cheat
    }
    fn set_c(self: &Rc<Self>, c: char) {
        self.borrow_mut().cheat = c;
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
    fn set_min_steps(self: &Rc<Self>, steps: usize) {
        self.borrow_mut().min_steps_to = steps;
    }
    fn reset_min_steps_and_cheat(self: &Rc<Self>) {
        self.set_min_steps(usize::MAX);
        self.set_c(' ');
    }
    fn step(self: &Rc<Self>, cur_steps: usize, pre_c: char, dp: bool, di: bool) -> Option<usize> {
        let np = !dp;
        let ind = if di { cur_steps * 2 } else { 0 };
        let p = format!("{:ind$}{},{}", "", self.get_x(), self.get_y());
        if self.get_min_steps_to() < cur_steps {
            let _ = np || () == println!("{p} {} < {}", self.get_min_steps_to(), cur_steps);
            return None;
        }
        if self.cant_step_to(pre_c) {
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
        for w in [self.get_r(), self.get_d(), self.get_l(), self.get_u()] {
            if let Some(s) = w.upgrade() {
                let _ = np
                    || () == println!("{p} trying {},{} {}", s.get_x(), s.get_y(), cur_steps + 1);
                if let Some(steps) = s.step(cur_steps + 1, self.get_c(), dp, di) {
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
    fn get_map_string(&self) -> String {
        self.spaces
            .iter()
            .map(|r| r.iter().map(Space::get_k).join(""))
            .join("\n")
    }
    fn get_map_string_w_cheats(&self) -> String {
        self.spaces
            .iter()
            .map(|r| {
                r.iter()
                    .map(|s| {
                        if s.get_c() != ' ' {
                            s.get_c()
                        } else {
                            s.get_k()
                        }
                    })
                    .join("")
            })
            .join("\n")
    }
    fn reset(&self) {
        for r in &self.spaces {
            for s in r {
                s.reset_min_steps_and_cheat();
            }
        }
    }
    fn get_steps_s_to_e(&self, do_print: bool, do_ind: bool) -> Option<(usize, String)> {
        if let Some(start) = self.start.upgrade() {
            if let Some(ret) = start.step(0, ' ', do_print, do_ind) {
                let map = self.get_map_string_w_cheats();
                self.reset();
                return Some((ret, map));
            }
        }
        self.reset();
        None
    }
    fn get_steps_s_to_e_cheat_pos(&self, c1: Pos, c2: Pos) -> Option<(usize, String)> {
        for (Pos { x, y }, c) in [(c1, '1'), (c2, '2')] {
            if x >= self.width || y >= self.height {
                self.reset();
                return None;
            }
            self.spaces[y][x].set_c(c);
        }
        self.get_steps_s_to_e(false, false)
    }
    fn get_steps_s_to_e_cheat(&self, c1: Weak<Space>, c2: Weak<Space>) -> Option<(usize, String)> {
        for (s, c) in [(c1, '1'), (c2, '2')] {
            if let Some(s) = s.upgrade() {
                s.set_c(c);
            } else {
                self.reset();
                return None;
            }
        }
        self.get_steps_s_to_e(false, false)
    }
    fn get_steps_with_cheats(&self) -> HashMap<Option<isize>, usize> {
        let (base, map) = self.get_steps_s_to_e(false, false).unwrap();
        let mut v = Vec::new();
        for r in &self.spaces {
            for s in r {
                v.push(self.get_steps_s_to_e_cheat(s.get_w(), s.get_r()));
                v.push(self.get_steps_s_to_e_cheat(s.get_w(), s.get_d()));
                v.push(self.get_steps_s_to_e_cheat(s.get_w(), s.get_l()));
                v.push(self.get_steps_s_to_e_cheat(s.get_w(), s.get_u()));
            }
        }
        v.sort();

        println!("{map}");
        for (v, s) in v.iter().flatten() {
            if v == &20 {
                println!("{s}");
            }
        }
        v.into_iter()
            .map(|c| c.map(|c| c.0))
            .map(|c| c.map(|c| base as isize - c as isize))
            .counts()
    }
}
fn run1() {
    let s = Instant::now();
    let m = Map::new(e1());
    let steps = m.get_steps_with_cheats();
    println!("{:?}", s.elapsed());
    for (&s, &c) in steps.iter().sorted_by_key(|&(s, _)| s) {
        println!("{c:<3} saved {s:?}");
    }
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
        assert_eq!(84, m.get_steps_s_to_e(false, false).unwrap().0);
    }
    #[test]
    fn e1_cheat_pos1() {
        let m = Map::new(e1());
        assert_eq!(
            72,
            m.get_steps_s_to_e_cheat_pos(Pos { x: 8, y: 1 }, Pos { x: 9, y: 1 })
                .unwrap()
                .0
        );
    }
    #[test]
    fn e1_cheat_pos2() {
        let m = Map::new(e1());
        assert_eq!(
            64,
            m.get_steps_s_to_e_cheat_pos(Pos { x: 10, y: 7 }, Pos { x: 11, y: 7 })
                .unwrap()
                .0
        );
    }
    #[test]
    fn e1_cheat1() {
        let m = Map::new(e1());
        assert_eq!(
            72,
            m.get_steps_s_to_e_cheat(m.spaces[1][8].get_w(), m.spaces[1][8].get_r())
                .unwrap()
                .0
        );
    }
    #[test]
    fn e1_cheat2() {
        let m = Map::new(e1());
        assert_eq!(
            64,
            m.get_steps_s_to_e_cheat(m.spaces[7][10].get_w(), m.spaces[7][10].get_r())
                .unwrap()
                .0
        );
    }
    #[test]
    fn e1_cheat3() {
        let m = Map::new(e1());
        assert_eq!(
            46,
            m.get_steps_s_to_e_cheat(m.spaces[8][8].get_w(), m.spaces[8][8].get_d())
                .unwrap()
                .0
        );
    }
    #[test]
    fn e1_cheat4() {
        let m = Map::new(e1());
        assert_eq!(
            20,
            m.get_steps_s_to_e_cheat(m.spaces[7][6].get_w(), m.spaces[7][6].get_l())
                .unwrap()
                .0
        );
    }
    #[test]
    fn d_s_to_e() {
        let res = run_with_big_stack_and_wait_and_ret(|| {
            let m = Map::new(d());
            m.get_steps_s_to_e(false, false)
        });
        assert_eq!(9348, res.unwrap().0);
    }
}
