#![allow(dead_code)]
use std::{collections::HashMap, time::Instant};

fn e1() -> &'static str {
    "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
"
}
fn e2() -> &'static str {
    "\
Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176
"
}
fn e3() -> &'static str {
    "\
Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450
"
}
fn e4() -> &'static str {
    "\
Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"
}
fn d() -> &'static str {
    include_str!("input.txt")
}

#[derive(Debug)]
struct Btn {
    kind: char,
    dx: isize,
    dy: isize,
}
impl Btn {
    const MAX_PRESSES_PER_BUTTON: isize = 100;
    const COST_A: isize = 3;
    const COST_B: isize = 1;
    fn new(d: &str) -> Self {
        let s = d.split(' ').collect::<Vec<_>>();
        let k = s[1].split(':').next().expect("Btn kind").chars().next().expect("char Kind");
        let mut xy = s[2..4].iter().map(|v|{
            let mut c = v.split(',').next().expect("xy comma").chars();
            let a = c.next().expect("axis");
            let d = c.as_str().parse::<isize>().expect("parse delta");
            (a, d)
        }).collect::<HashMap<_, _>>();
        Self {
            kind: k,
            dx: xy.remove(&'X').expect("Btn get X"),
            dy: xy.remove(&'Y').expect("Btn get Y"),
        }
    }
}
#[derive(Debug)]
struct Prize {
    x: isize,
    y: isize,
}
impl Prize {
    fn new(d: &str, m: isize) -> Self {
        let mut xy = d.split(' ').skip(1).map(|v|{
            let mut c = v.split(',').next().expect("xy comma").chars();
            let a = c.next().expect("axis");
            c.next();
            let p = c.as_str().parse::<isize>().expect("parse point") + m;
            (a, p)
        }).collect::<HashMap<_, _>>();
        Self {
            x: xy.remove(&'X').expect("Prize get X"),
            y: xy.remove(&'Y').expect("Prize get Y"),
        }
    }
}
#[derive(Debug)]
struct Machine {
    a: Btn,
    b: Btn,
    p: Prize,
}
impl Machine {
    fn new(d: &str, m: isize) -> Self {
        let l = d.lines().filter(|l|!l.is_empty()).collect::<Vec<_>>();
        let mut ab = l[0..2].iter().map(|ab| {
            let b = Btn::new(ab);
            (b.kind, b)
        }).collect::<HashMap<_, _>>();
        let p = Prize::new(l[2], m);
        Self {
            a: ab.remove(&'A').expect("get A"),
            b: ab.remove(&'B').expect("get B"),
            p,
        }
    }
    fn cheapest(&self) -> Option<(isize, isize)> {
        let mut last_a = 0;
        for bp in (0..=Btn::MAX_PRESSES_PER_BUTTON).rev() {
            let x = self.b.dx * bp;
            let y = self.b.dy * bp;
            if x > self.p.x || y > self.p.y {
                continue;
            }
            for ap in last_a..=Btn::MAX_PRESSES_PER_BUTTON {
                let x = self.a.dx * ap + x;
                let y = self.a.dy * ap + y;
                if x == self.p.x && y == self.p.y {
                    return Some((ap, bp));
                } else if x < self.p.x || y < self.p.y {
                    last_a = ap;
                } else {
                    break;
                }
            }
        }
        None
    }
    fn get_cost((a, b): (isize, isize)) -> isize {
        a * Btn::COST_A + b * Btn::COST_B
    }
    fn cheapest2(&self) -> Option<(isize, isize)> {
        let get_max = |btn: &Btn| {
            let max_x = self.p.x / btn.dx + 1;
            let max_y = self.p.y / btn.dy + 1;
            std::cmp::min(max_x, max_y)
        };
        let max_a = get_max(&self.a);
        let max_b = get_max(&self.b);
        let mut last_a = 0;
        for bp in (0..max_b).rev() {
            let x = self.b.dx * bp;
            let y = self.b.dy * bp;
            if x > self.p.x || y > self.p.y {
                continue;
            }
            for ap in last_a..max_a {
                let x = self.a.dx * ap + x;
                let y = self.a.dy * ap + y;
                if x == self.p.x && y == self.p.y {
                    return Some((ap, bp));
                } else if x < self.p.x || y < self.p.y {
                    last_a = ap;
                } else {
                    break;
                }
            }
        }
        None
    }
    fn cheapest3(&self) -> Option<(isize, isize)> {
        // a = (x_b * y_p - x_p * y_b) / (x_b * y_a - x_a * y_b)
        // b = (x_p - x_a * a) / x_b
        let x_a = self.a.dx;
        let x_b = self.b.dx;
        let x_p = self.p.x;
        let y_a = self.a.dy;
        let y_b = self.b.dy;
        let y_p = self.p.y;

        let a_n = x_b * y_p - x_p * y_b;
        let a_d = x_b * y_a - x_a * y_b;
        let a = a_n / a_d;
        let a_r = a_n % a_d;

        let b_n = x_p - x_a * a;
        let b_d = x_b;
        let b = b_n / b_d;
        let b_r = b_n % b_d;

        if a_r != 0 || b_r != 0 {
            None
        } else {
            Some((a, b))
        }
    }
}
fn cost_multiple(d: &str, m: isize) -> isize {
    d.split("\n\n").filter_map(|ls| Machine::new(ls, m).cheapest3()).map(Machine::get_cost).sum::<isize>()
}
fn main() {
    let s = Instant::now();
    println!("{} {:?}", cost_multiple(d(), 10000000000000), s.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test] fn e1_c() { assert_eq!(Some((80, 40)), Machine::new(e1(), 0).cheapest3()) }
    #[test] fn e2_c() { assert_eq!(None, Machine::new(e2(), 0).cheapest3()) }
    #[test] fn e3_c() { assert_eq!(Some((38, 86)), Machine::new(e3(), 0).cheapest3()) }
    #[test] fn e4_c() { assert_eq!(None, Machine::new(e4(), 0).cheapest3()) }
    #[test] fn e1_c_c() { assert_eq!(280, Machine::get_cost(Machine::new(e1(), 0).cheapest3().unwrap())) }
    #[test] fn e3_c_c() { assert_eq!(200, Machine::get_cost(Machine::new(e3(), 0).cheapest3().unwrap())) }
    #[test] fn d_c_c() { assert_eq!(34787, cost_multiple(d(), 0)) }
    #[test] fn d_c_c_2() { assert_eq!(85644161121698, cost_multiple(d(), 10000000000000)) }
}
