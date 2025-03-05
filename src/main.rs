#![allow(dead_code)]

use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
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
    fn cant_step_to(self: &Rc<Self>) -> bool {
        self.borrow().kind == '#'
    }
    fn at_start(self: &Rc<Self>) -> bool {
        self.borrow().kind == 'S'
    }
    fn at_end(self: &Rc<Self>) -> bool {
        self.borrow().kind == 'E'
    }
    fn get_x(self: &Rc<Self>) -> usize {
        self.borrow().x
    }
    fn get_y(self: &Rc<Self>) -> usize {
        self.borrow().y
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
    fn step(self: &Rc<Self>, cur_steps: usize, do_print: bool, do_ind: bool) -> Option<usize> {
        let ind = if do_ind { cur_steps * 2 } else { 0 };
        if self.get_min_steps_to() < cur_steps {
            if do_print {
                println!(
                    "{:ind$}{},{} {} < {}",
                    "",
                    self.get_x(),
                    self.get_y(),
                    self.get_min_steps_to(),
                    cur_steps
                );
            }
            return None;
        }
        if self.cant_step_to() {
            if do_print {
                println!("{:ind$}{},{} cant_step_to", "", self.get_x(), self.get_y());
            }
            return None;
        }
        if do_print {
            println!(
                "{:ind$}{},{} min_steps_to = {cur_steps}",
                "",
                self.get_x(),
                self.get_y()
            );
        }
        self.set_min_steps(cur_steps);
        if self.at_end() {
            if do_print {
                println!(
                    "{:ind$}{},{} at_end = {cur_steps}",
                    "",
                    self.get_x(),
                    self.get_y()
                );
            }
            return Some(cur_steps);
        }
        let mut costs = Vec::new();
        let sb = self.borrow();
        for w in [&sb.right, &sb.down, &sb.left, &sb.up] {
            if let Some(s) = w.upgrade() {
                if do_print {
                    println!(
                        "{:ind$}{},{} trying {},{} {}",
                        "",
                        self.get_x(),
                        self.get_y(),
                        s.get_x(),
                        s.get_y(),
                        cur_steps + 1
                    );
                }
                if let Some(steps) = s.step(cur_steps + 1, do_print, do_ind) {
                    costs.push(steps);
                }
            }
        }
        let ret = costs.into_iter().min();
        if do_print {
            println!(
                "{:ind$}{},{} returning cur_steps+1 == {} {:?} ",
                "",
                self.get_x(),
                self.get_y(),
                cur_steps + 1,
                ret
            );
        }
        ret
    }
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
        let ret = self.start.upgrade()?.step(0, do_print, do_ind);
        for r in &self.spaces {
            for s in r {
                s.reset_min_steps();
            }
        }
        ret
    }
}
fn run1() {
    let s = Instant::now();
    let m = Map::new(d());
    // println!("{}x{} {:?} {:?}", m.width, m.height, m.get_steps_s_to_e(true, false), s.elapsed()); // 9s (in release mode)
    println!("{}x{} {:?} {:?}", m.width, m.height, m.get_steps_s_to_e(false, false), s.elapsed()); // 5ms (in release mode)
}
fn run_with_big_stack_and_wait(f: fn()) {
    thread::Builder::new()
        .stack_size(1024 * 1024 * 1024)
        .spawn(f)
        .unwrap()
        .join()
        .unwrap();
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
}
