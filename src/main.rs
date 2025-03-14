#![allow(dead_code)]

use std::collections::HashMap;
use itertools::Itertools;

const fn e1() -> &'static str {
    "\
029A
980A
179A
456A
379A
"
}
const fn d() -> &'static str {
    include_str!("input.txt")
}

struct NumSeq<'a> {
    v: &'a str
}
impl<'a> NumSeq<'a> {
    fn new(seq: &'a str) -> Self {
        if seq.chars().all(|c| ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'].contains(&c)) {
            Self { v: seq }
        } else {
            panic!("invalid NumSeq: {seq}");
        }
    }
}
struct DirSeq<'a> {
    v: &'a str
}
impl<'a> DirSeq<'a> {
    fn new(seq: &'a str) -> Self {
        if seq.chars().all(|c| ['^', 'v', '>', '<', 'A'].contains(&c)) {
            Self { v: seq }
        } else {
            panic!("invalid DirSeq: {seq}");
        }
    }
}
enum PadType {
    Num,
    Dir,
}
struct Pad {
    x: usize,
    y: usize,
    pad: Vec<Vec<char>>,
    hm: HashMap<char, (usize, usize)>,
}
impl Pad {
    fn new(t: PadType) -> Self {
        match t {
            PadType::Dir => Self {
                x: 2,
                y: 0,
                hm: HashMap::from_iter([
                    ('A', (2, 0)),
                    ('^', (1, 0)),
                    ('<', (0, 1)),
                    ('v', (1, 1)),
                    ('>', (2, 1)),
                ]),
                pad: vec![
                    vec![' ', '^', 'A'],
                    vec!['<', 'v', '>']
                ],
            },
            PadType::Num => Self {
                x: 2,
                y: 3,
                hm: HashMap::from_iter([
                    ('A', (2, 3)),
                    ('0', (1, 3)),
                    ('1', (0, 2)),
                    ('2', (1, 2)),
                    ('3', (2, 2)),
                    ('4', (0, 1)),
                    ('5', (1, 1)),
                    ('6', (2, 1)),
                    ('7', (0, 0)),
                    ('8', (1, 0)),
                    ('9', (2, 0)),
                ]),
                pad: vec![
                    vec!['7', '8', '9'],
                    vec!['4', '5', '6'],
                    vec!['1', '2', '3'],
                    vec![' ', '0', 'A'],
                ],
            },
        }
    }
    fn get_dir_str(s: usize, d: usize, d1: char, d2: char) -> String {
        if s <= d {
            d1.to_string().repeat(d - s)
        } else {
            d2.to_string().repeat(s - d)
        }
    }
    fn expand(&mut self, num: char) -> String {
        let (to_x, to_y) = self.hm[&num];
        let dx = Self::get_dir_str(self.x, to_x, '>', '<');
        let dy = Self::get_dir_str(self.y, to_y, 'v', '^');
        self.x = to_x;
        self.y = to_y;
        dx + &dy + "A"
    }
    fn validate(&self) {
        if self.x >= self.pad[0].len() {
            panic!("bad x: {}", self.x);
        }
        if self.y >= self.pad.len() {
            panic!("bad y: {}", self.y);
        }
    }
    fn reduce(&mut self, dir: char) {
        match dir {
            '^' => self.y -= 1,
            'v' => self.y += 1,
            '<' => self.x -= 1,
            '>' => self.x += 1,
            _ => panic!("bad dir {dir}"),
        }
        self.validate();
    }
    fn get_pad_val(&self) -> char {
        self.pad[self.y][self.x]
    }
    fn reduce_seq_val(&mut self, seq: &DirSeq) -> String {
        seq.v.split_terminator('A')
            .map(|sub| {
                for dir in sub.chars() {
                    self.reduce(dir);
                }
                self.get_pad_val()
            })
            .join("")
    }
    fn reduce_seq_dir(seq: &DirSeq) -> String {
        Self::new(PadType::Dir).reduce_seq_val(seq)
    }
    fn reduce_seq_num(seq: &DirSeq) -> String {
        Self::new(PadType::Num).reduce_seq_val(seq)
    }
    fn reduce_seq_dir_n(seq: &DirSeq, n: usize) -> String {
        let mut res = seq.v.to_owned();
        for _ in 0..n {
            res = Self::reduce_seq_dir(&DirSeq::new(&res));
        }
        res
    }
    fn reduce_seq_dir_n_then_num(seq: &DirSeq, n: usize) -> String {
        let res = Self::reduce_seq_dir_n(seq, n);
        Self::reduce_seq_num(&DirSeq::new(&res))
    }
}

fn main() {
    let mut p = Pad::new(PadType::Num);
    println!("{} {} {} {}", p.expand('0'), p.expand('2'), p.expand('9'), p.expand('A'));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn reduce_dir_pad_1() {
        assert_eq!(
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A",
            Pad::new(PadType::Dir).reduce_seq_val(
                &DirSeq::new("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A")
            )
        );
    }
    #[test]
    fn reduce_dir_pad_2() {
        assert_eq!(
            "<A^A>^^AvvvA",
            Pad::new(PadType::Dir).reduce_seq_val(&DirSeq::new("v<<A>>^A<A>AvA<^AA>A<vAAA>^A"))
        );
    }
    #[test]
    fn reduce_dir_pad_3() {
        assert_eq!(
            "<A^A>^^AvvvA",
            Pad::reduce_seq_dir(&DirSeq::new("v<<A>>^A<A>AvA<^AA>A<vAAA>^A"))
        );
    }
    #[test]
    fn reduce_num_pad_1() {
        assert_eq!("029A", Pad::new(PadType::Num).reduce_seq_val(&DirSeq::new("<A^A>^^AvvvA")));
    }
    #[test]
    fn reduce_num_pad_2() {
        assert_eq!("029A", Pad::reduce_seq_num(&DirSeq::new("<A^A>^^AvvvA")));
    }
    #[test]
    fn reduce_combo_1() {
        assert_eq!(
            "029A",
            Pad::reduce_seq_dir_n_then_num(
                &DirSeq::new("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"),
                2
            )
        );
    }
    #[test]
    fn reduce_combo_2() {
        assert_eq!(
            "980A",
            Pad::reduce_seq_dir_n_then_num(
                &DirSeq::new("<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A"),
                2
            )
        );
    }
    #[test]
    fn reduce_combo_3() {
        assert_eq!(
            "179A",
            Pad::reduce_seq_dir_n_then_num(
                &DirSeq::new("<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"),
                2
            )
        );
    }
    #[test]
    fn reduce_combo_4() {
        assert_eq!(
            "456A",
            Pad::reduce_seq_dir_n_then_num(
                &DirSeq::new("<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A"),
                2
            )
        );
    }
    #[test]
    fn reduce_combo_5() {
        assert_eq!(
            "379A",
            Pad::reduce_seq_dir_n_then_num(
                &DirSeq::new("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"),
                2
            )
        );
    }
}
