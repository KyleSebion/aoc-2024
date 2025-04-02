#![allow(dead_code)]

use std::{iter, time::Duration};
fn e1() -> (&'static str, isize, isize) {
    (
        "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
",
        11,
        7,
    )
}
fn e1_100() -> &'static str {
    "\
......2..1.
...........
1..........
.11........
.....1.....
...12......
.1....1....
"
}
fn d() -> (&'static str, isize, isize) {
    (
        include_str!("input.txt"),
        101,
        103,
    )
}
#[derive(Debug)]
struct Bot {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    w: isize,
    h: isize,
}
impl Bot {
    fn new(d: &str, w: isize, h: isize) -> Self {
        let d = d
            .split(&['p', '=', ',', ' ', 'v'][..])
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse::<isize>().ok())
            .collect::<Vec<_>>();
        Self {
            x: d[0],
            y: d[1],
            dx: d[2],
            dy: d[3],
            w,
            h,
        }
    }
    fn parse_bot_list(d: &str, w: isize, h: isize) -> Vec<Self> {
        d.lines().map(|l| Self::new(l, w, h)).collect()
    }
    fn parse_tuple((d, w, h): (&str, isize, isize)) -> Vec<Self> {
        Self::parse_bot_list(d, w, h)
    }
    fn step_axis(p: isize, dp: isize, m: isize) -> isize {
        let p = p + dp;
        if p < 0 {
            p + m
        } else if p >= m {
            p - m
        } else {
            p
        }
    }
    fn step(b: &mut [Self]) {
        for b in b {
            b.x = Self::step_axis(b.x, b.dx, b.w);
            b.y = Self::step_axis(b.y, b.dy, b.h);
        }
    }
    fn step_n(b: &mut [Self], n: isize) {
        for _ in 0..n {
            Self::step(b);
        }
    }
    fn map_str(b: &[Self]) -> String {
        let mut m = vec![vec![' '; b[0].w as usize]; b[0].h as usize];
        for b in b {
            m[b.y as usize][b.x as usize] = match m[b.y as usize][b.x as usize] {
                ' ' => '1',
                '1' => '2',
                '2' => '3',
                '3' => '4',
                '4' => '5',
                '5' => '6',
                '6' => '7',
                '7' => '8',
                '8' => '9',
                _ => 'X',
            };
        }
        String::from_iter(m.into_iter().flat_map(|r| r.into_iter().chain(iter::once('\n'))))
    }
    fn get_safety_factor(b: &[Self]) -> isize {
        let w = b[0].w;
        let h = b[0].h;
        let mid_w = w / 2;
        let mid_h = h / 2;
        let s_rng_w = [0..mid_w, (mid_w + 1)..(w)];
        let s_rng_h = [0..mid_h, (mid_h + 1)..(h)];
        let mut s_sum = vec![vec![0, 0]; 2];
        for b in b {
            let x = s_rng_w.iter().position(|r| r.contains(&b.x));
            let y = s_rng_h.iter().position(|r| r.contains(&b.y));
            if let (Some(x), Some(y)) = (x, y) { s_sum[y][x] += 1; }
        }
        s_sum.into_iter().flatten().product()
    }
}
fn main() {
    let mut b = Bot::parse_tuple(d());
    for i in 1.. {
        Bot::step_n(&mut b, 1);
        if i % 103 == 84 {
            println!("{}{i}", Bot::map_str(&b));
            // println!("\x1B[2J\x1B[H{}{i}", Bot::map_str(&b));
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    // println!("{}", Bot::get_safety_factor(&b));
}
#[cfg(test)]
mod test {
    use super::*;
    fn get_stepped_bots(d: (&str, isize, isize), n: isize) -> Vec<Bot> {
        let mut b = Bot::parse_tuple(d);
        Bot::step_n(&mut b, n);
        b
    }
    fn get_stepped_bot(d: (&str, isize, isize), i: usize, n: isize) -> Bot {
        get_stepped_bots(d, n).remove(i)
    }
    fn get_stepped_bots_map(d: (&str, isize, isize), n: isize) -> String {
        Bot::map_str(&get_stepped_bots(d, n))
    }
    fn get_stepped_bots_safety(d: (&str, isize, isize), n: isize) -> isize {
        Bot::get_safety_factor(&get_stepped_bots(d, n))
    }
    #[test] fn e1_b10_0() { let b = get_stepped_bot(e1(), 10, 0); assert_eq!((2, 4), (b.x, b.y)) }
    #[test] fn e1_b10_1() { let b = get_stepped_bot(e1(), 10, 1); assert_eq!((4, 1), (b.x, b.y)) }
    #[test] fn e1_b10_2() { let b = get_stepped_bot(e1(), 10, 2); assert_eq!((6, 5), (b.x, b.y)) }
    #[test] fn e1_b10_3() { let b = get_stepped_bot(e1(), 10, 3); assert_eq!((8, 2), (b.x, b.y)) }
    #[test] fn e1_b10_4() { let b = get_stepped_bot(e1(), 10, 4); assert_eq!((10, 6), (b.x, b.y)) }
    #[test] fn e1_b10_5() { let b = get_stepped_bot(e1(), 10, 5); assert_eq!((1, 3), (b.x, b.y)) }
    #[test] fn e1_100_str() { assert_eq!(e1_100(), get_stepped_bots_map(e1(), 100)) }
    #[test] fn e1_100_safety() { assert_eq!(12, get_stepped_bots_safety(e1(), 100)) }
    #[test] fn d1_100_safety() { assert_eq!(224438715, get_stepped_bots_safety(d(), 100)) }
}
