#![allow(dead_code, clippy::unit_cmp)]

use itertools::Itertools;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::collections::VecDeque;
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
    fn get_xy(self: &Rc<Self>) -> (usize, usize) {
        (self.get_x(), self.get_y())
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
#[derive(Debug)]
struct CheatPoints {
    start: Pos,
    end: Pos,
    saved: usize,
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
    fn get_map_string_w_num(&self) -> String {
        const DEF_COLORS: &str = "\x1b[38;2;217;217;217m\x1b[48;2;0;0;0m";
        DEF_COLORS.to_owned()
            + &self
                .spaces
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|s| {
                            if s.get_k() == '#' {
                                "██████".to_owned()
                            } else {
                                let (pre, suf) = if s.at_start() || s.at_end() {
                                    ("\x1b[48;2;0;255;0m\x1b[38;2;0;0;0m", DEF_COLORS)
                                } else {
                                    ("", "")
                                };
                                format!("{pre}{:^6}{suf}", s.get_min_steps_to())
                            }
                        })
                        .join("")
                })
                .join("\n")
    }
    fn get_map_string_w_num_tab(&self) -> String {
        self.spaces
            .iter()
            .map(|r| {
                r.iter()
                    .map(|s| {
                        if s.get_k() == '#' {
                            s.get_k().to_string()
                        } else {
                            s.get_min_steps_to().to_string()
                        }
                    })
                    .join("\t")
            })
            .join("\n")
    }
    fn change_steps_from_s_to_steps_from_e(&self) {
        let e = self.end.upgrade().unwrap().get_min_steps_to();
        for r in &self.spaces {
            for s in r {
                let n = e - s.get_min_steps_to();
                s.set_min_steps(n);
            }
        }
    }
    fn reset(&self) {
        for r in &self.spaces {
            for s in r {
                s.reset_min_steps_and_cheat();
            }
        }
    }
    fn get_steps_s_to_e(&self, do_print: bool, do_ind: bool) -> Option<(usize, String)> {
        let res = self.get_steps_s_to_e_no_reset(do_print, do_ind);
        self.reset();
        res
    }
    fn get_steps_s_to_e_no_reset(&self, do_print: bool, do_ind: bool) -> Option<(usize, String)> {
        if let Some(start) = self.start.upgrade() {
            if let Some(ret) = start.step(0, ' ', do_print, do_ind) {
                // let map = self.get_map_string_w_cheats();
                let map = "".to_owned();
                // println!("{map}");
                return Some((ret, map));
            }
        }
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
        for (s, c) in [(&c1, '1'), (&c2, '2')] {
            if let Some(s) = s.upgrade() {
                if c == '1' && s.get_k() == '#' {
                    let surrounded_by_walls = [s.get_r(), s.get_d(), s.get_l(), s.get_r()]
                        .into_iter()
                        .filter_map(|c| c.upgrade())
                        .map(|c| c.get_k())
                        .all(|c| c == '#');
                    if !surrounded_by_walls {
                        s.set_c(c);
                    } else {
                        self.reset();
                        return None;
                    }
                } else if c == '2' && s.get_k() != '#' {
                    s.set_c(c);
                } else {
                    self.reset();
                    return None;
                }
            } else {
                self.reset();
                return None;
            }
        }
        // println!(
        //     "{},{}",
        //     c1.upgrade().unwrap().get_x(),
        //     c1.upgrade().unwrap().get_y()
        // );
        // println!("{}\n", self.get_map_string_w_cheats());
        self.get_steps_s_to_e(false, false)
    }
    fn get_saved_steps_with_cheats(&self) -> HashMap<Option<usize>, usize> {
        let (base, _) = self.get_steps_s_to_e(false, false).unwrap();
        let mut v = Vec::new();
        for r in &self.spaces {
            for s in r {
                if s.get_k() == '#' {
                    if (1..self.width - 1).contains(&s.get_x()) {
                        v.push(self.get_steps_s_to_e_cheat(s.get_w(), s.get_r()));
                        v.push(self.get_steps_s_to_e_cheat(s.get_w(), s.get_l()));
                    }
                    if (1..self.height - 1).contains(&s.get_y()) {
                        v.push(self.get_steps_s_to_e_cheat(s.get_w(), s.get_d()));
                        v.push(self.get_steps_s_to_e_cheat(s.get_w(), s.get_u()));
                    }
                }
            }
        }
        v.sort();
        v.into_iter()
            .map(|c| c.map(|c| c.0))
            .map(|c| c.map(|c| base - c))
            .counts()
    }
    fn get_count_of_cheats_that_save_at_least_100_steps(&self) -> usize {
        self.get_saved_steps_with_cheats()
            .into_iter()
            .filter_map(|(s, c)| {
                if let Some(s) = s {
                    if s >= 100 {
                        return Some(c);
                    }
                }
                None
            })
            .sum::<usize>()
    }
    fn get_path(&self) -> VecDeque<Rc<Space>> {
        let mut path = VecDeque::new();
        let mut cur_or_none = self.start.upgrade();
        let mut next_step = 0;
        while let Some(cur) = cur_or_none {
            next_step = cur.get_min_steps_to() + 1;
            let next = [cur.get_r(), cur.get_d(), cur.get_l(), cur.get_u()]
                .into_iter()
                .filter_map(|n| n.upgrade())
                .filter(|n| n.get_min_steps_to() == next_step)
                .collect_vec();
            if next.len() > 1 {
                panic!("{:?}", next.into_iter().map(|v| v.get_xy()).collect_vec());
            }
            path.push_back(cur);
            cur_or_none = next.into_iter().nth(0);
        }
        if next_step - 1 != self.end.upgrade().unwrap().get_min_steps_to() {
            panic!("next_step {next_step} != end");
        }
        path
    }
    fn get_cheat_area(len: usize) -> Vec<(usize, Pos)> {
        let mut area = Vec::new();
        for y in 0..len {
            for x in 0..len - y {
                if x == 0 && y == 0 {
                    continue;
                }
                let count = x + y;
                area.push((count, Pos { x, y }));
                area.push((
                    count,
                    Pos {
                        x: usize::MIN.wrapping_sub(x),
                        y: usize::MIN.wrapping_sub(y),
                    },
                ));
                if x > 0 && y > 0 {
                    area.push((
                        count,
                        Pos {
                            x,
                            y: usize::MIN.wrapping_sub(y),
                        },
                    ));
                    area.push((
                        count,
                        Pos {
                            x: usize::MIN.wrapping_sub(x),
                            y,
                        },
                    ));
                }
            }
        }
        area
    }
    fn get_cheat_spaces(&self, c_len: usize, x: usize, y: usize) -> Vec<(usize, Rc<Space>)> {
        Self::get_cheat_area(c_len)
            .into_iter()
            .filter_map(|(s, Pos { x: dx, y: dy })| {
                let x = x.wrapping_add(dx);
                let y = y.wrapping_add(dy);
                if x < self.width && y < self.height {
                    Some((s, Rc::clone(&self.spaces[y][x])))
                } else {
                    None
                }
            })
            .collect()
    }
    fn get_cheat_points(&self, c_len: usize) -> Vec<CheatPoints> {
        self.get_steps_s_to_e_no_reset(false, false);
        let mut cp = Vec::new();
        let mut path = self.get_path();
        while let Some(p) = path.pop_front() {
            let step0_xy = p.get_xy();
            let step_num = p.get_min_steps_to();
            let mut options = Vec::new();
            for step1 in [p.get_r(), p.get_d(), p.get_l(), p.get_u()]
                .into_iter()
                .filter_map(|v| v.upgrade())
            {
                let step1_xy = step1.get_xy();
                // if step1.get_k() != '#' {
                //     continue;
                // }
                for (add_steps, c_end) in self.get_cheat_spaces(c_len, step1_xy.0, step1_xy.1) {
                    if c_end.get_k() != '#' && c_end.get_min_steps_to() >= step_num + 1 + add_steps
                    {
                        options.push(CheatPoints {
                            start: Pos {
                                x: step0_xy.0,
                                y: step0_xy.1,
                            },
                            end: Pos {
                                x: c_end.get_x(),
                                y: c_end.get_y(),
                            },
                            saved: c_end.get_min_steps_to() - step_num - 1 - add_steps,
                        });
                    }
                }
            }
            for (_, group) in options
                .into_iter()
                .sorted_by_key(|c| (c.end.x, c.end.y))
                .chunk_by(|c| (c.end.x, c.end.y))
                .into_iter()
            {
                let max = group.max_by_key(|c| c.saved).unwrap();
                cp.push(max);
            }
        }
        cp
    }
}
fn run1() {
    let s = Instant::now();
    let res = Map::new(d())
        .get_cheat_points(20)
        .into_iter()
        .counts_by(|c| c.saved >= 100)[&true];
    println!("{res} {:?}", s.elapsed());
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
    fn e1_all_cheats() {
        let m = Map::new(e1());
        let vr = vec![
            "377 saved None",
            "19  saved Some(0)",
            "14  saved Some(2)",
            "14  saved Some(4)",
            "2   saved Some(6)",
            "4   saved Some(8)",
            "2   saved Some(10)",
            "3   saved Some(12)",
            "1   saved Some(20)",
            "1   saved Some(36)",
            "1   saved Some(38)",
            "1   saved Some(40)",
            "1   saved Some(64)",
        ];
        let steps = m.get_saved_steps_with_cheats();
        for (i, (&s, &c)) in steps.iter().sorted_by_key(|&(s, _)| s).enumerate() {
            assert_eq!(vr[i], format!("{c:<3} saved {s:?}"));
        }
    }
    #[test]
    fn e1_all_cheats2() {
        let m = Map::new(e1());
        let vr = [
            "83  saved 0",
            "14  saved 2",
            "14  saved 4",
            "2   saved 6",
            "4   saved 8",
            "2   saved 10",
            "3   saved 12",
            "1   saved 20",
            "1   saved 36",
            "1   saved 38",
            "1   saved 40",
            "1   saved 64",
        ];
        let steps = m.get_cheat_points(2).into_iter().counts_by(|c| c.saved);
        for (i, (&s, &c)) in steps.iter().sorted_by_key(|&(s, _)| s).enumerate() {
            assert_eq!(vr[i], format!("{c:<3} saved {s:?}"));
        }
    }
    #[test]
    fn e1_all_cheats_p2() {
        let need = "\
There are 32 cheats that save 50 picoseconds.
There are 31 cheats that save 52 picoseconds.
There are 29 cheats that save 54 picoseconds.
There are 39 cheats that save 56 picoseconds.
There are 25 cheats that save 58 picoseconds.
There are 23 cheats that save 60 picoseconds.
There are 20 cheats that save 62 picoseconds.
There are 19 cheats that save 64 picoseconds.
There are 12 cheats that save 66 picoseconds.
There are 14 cheats that save 68 picoseconds.
There are 12 cheats that save 70 picoseconds.
There are 22 cheats that save 72 picoseconds.
There are 4 cheats that save 74 picoseconds.
There are 3 cheats that save 76 picoseconds.";
        let res = Map::new(e1())
            .get_cheat_points(20)
            .iter()
            .counts_by(|c| c.saved)
            .into_iter()
            .filter(|v| v.0 >= 50)
            .sorted_by_key(|v| v.0)
            .map(|v| format!("There are {} cheats that save {} picoseconds.", v.1, v.0))
            .join("\n");
        assert_eq!(need, res);
    }
    #[test]
    fn d_s_to_e() {
        let res = run_with_big_stack_and_wait_and_ret(|| {
            let m = Map::new(d());
            m.get_steps_s_to_e(false, false)
        });
        assert_eq!(9348, res.unwrap().0);
    }
    #[test]
    fn d_count() {
        let res = run_with_big_stack_and_wait_and_ret(|| {
            let m = Map::new(d());
            m.get_count_of_cheats_that_save_at_least_100_steps()
        });
        assert_eq!(1321, res);
    }
    #[test]
    fn d_count2() {
        let res = run_with_big_stack_and_wait_and_ret(|| {
            Map::new(d())
                .get_cheat_points(2)
                .into_iter()
                .counts_by(|c| c.saved >= 100)[&true]
        });
        assert_eq!(1321, res);
    }
    #[test]
    fn d_count_p2() {
        let res = run_with_big_stack_and_wait_and_ret(|| {
            Map::new(d())
                .get_cheat_points(20)
                .into_iter()
                .counts_by(|c| c.saved >= 100)[&true]
        });
        assert_eq!(971737, res);
    }
}
