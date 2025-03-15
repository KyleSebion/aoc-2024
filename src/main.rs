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
    fn parse_to_num(&self) -> usize {
        self.v.trim_start_matches('0').trim_end_matches('A').parse().expect("parse_to_num failed")
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
                    (' ', (0, 0)),
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
                    (' ', (0, 3)),
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
    fn goes_out_of_bounds(&mut self, fr_y: usize, to_x: usize) -> bool {
        let (sp_x, sp_y) = self.hm[&' '];
        fr_y == sp_y && to_x == sp_x
    }
    fn expand(&mut self, num: char) -> String {
        let (to_x, to_y) = self.hm[&num];
        let goes_oob = self.goes_out_of_bounds(self.y, to_x);
        let dx = Self::get_dir_str(self.x, to_x, '>', '<');
        let dy = Self::get_dir_str(self.y, to_y, 'v', '^');
        self.x = to_x;
        self.y = to_y;
        if goes_oob {
            dy + &dx + "A"
        } else {
            dx + &dy + "A"
        }
    }
    fn expand_dir_seq_val(&mut self, seq: &DirSeq) -> String {
        seq.v.chars().map(|c| self.expand(c)).join("")
    }
    fn expand_num_seq_val(&mut self, seq: &NumSeq) -> String {
        seq.v.chars().map(|c| self.expand(c)).join("")
    }
    fn expand_seq_dir(seq: &DirSeq) -> String {
        Pad::new(PadType::Dir).expand_dir_seq_val(seq)
    }
    fn expand_seq_num(seq: &NumSeq) -> String {
        Pad::new(PadType::Num).expand_num_seq_val(seq)
    }
    fn expand_seq_dir_n(seq: &DirSeq, n: usize) -> String {
        let mut res = seq.v.to_owned();
        for _ in 0..n {
            res = Self::expand_seq_dir(&DirSeq::new(&res));
        }
        res
    }
    fn expand_seq_num_then_dir_n(seq: &NumSeq, n: usize) -> String {
        let res = Self::expand_seq_num(seq);
        Self::expand_seq_dir_n(&DirSeq::new(&res), n)
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
    fn get_code_complexity(code: &NumSeq) -> usize {
        Pad::expand_seq_num_then_dir_n(code, 2).len() * code.parse_to_num()
    }
    fn get_codes_complexity(codes: &str) -> usize {
        codes.lines().map(|code| Self::get_code_complexity(&NumSeq::new(code))).sum()
    }
}

fn main() {
    println!("1 {}", Pad::expand_seq_num_then_dir_n(&NumSeq::new("179A"), 2));
    // have: 1 <<vAA>A>^AAvA<^A>AvA^A<<vA>>^AAvA^A<vA>^AA<A>A<<vA>A>^AAAvA<^A>A  before oob change
    // have: 1 v<<A>>^A<vA<A>>^AAvAA<^A>Av<<A>>^AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A  after oob change
    // need:   <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

    println!("2 {}", Pad::expand_seq_num(&NumSeq::new("179A"))); // 2 <<^A ^^A >>A vvvA
    println!("3 {}", Pad::expand_seq_dir(&DirSeq::new("<<^A^^A>>AvvvA"))); // 3 <<vAA>^A>A<AA>AvAA^A<vAAA>^A
    println!("4 {}", Pad::expand_seq_dir(&DirSeq::new("<<vAA>^A>A<AA>AvAA^A<vAAA>^A"))); // 4 <<vAA>A>^AAvA<^A>AvA^A<<vA>>^AAvA^A<vA>^AA<A>A<<vA>A>^AAAvA<^A>A

    println!("5b {}", Pad::reduce_seq_dir(&DirSeq::new("<<vAA>A>^AAvA<^A>AvA^A<<vA>>^AAvA^A<vA>^AA<A>A<<vA>A>^AAAvA<^A>A")));        // 5b <<vAA>^A>A<AA>AvAA^A<vAAA>^A
                                                                                                                                     //       <<^A^^A>>AvvvA
                                                                                                                                     //    << results in hover over space
                                                                                                                                     //    needs to be v<<A
    println!("5g {}", Pad::reduce_seq_dir(&DirSeq::new("<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A")));    // 5g <A v<A A >>^A <AA>AvAA^A<vAAA>^A
                                                                                                                                     //     ^   < <    A  ^^ A >> A  vvv  A
    println!("6b {}", Pad::reduce_seq_dir(&DirSeq::new("<<vAA>^A>A<AA>AvAA^A<vAAA>^A")));    // 6b <<^A^^A>>AvvvA
    println!("6g {}", Pad::reduce_seq_dir(&DirSeq::new("<Av<AA>>^A<AA>AvAA^A<vAAA>^A")));    // 6g ^<<A^^A>>AvvvA

    println!("7b {}", Pad::expand_seq_dir(&DirSeq::new("<")));    // <<vA

    println!("{}", Pad::get_codes_complexity(d()));
    // 179008 too high
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
    #[test]
    fn expand_combo_1() {
        assert_eq!(
            "<vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A",
            Pad::expand_seq_num_then_dir_n(
                &NumSeq::new("029A"),
                2
            )
        );
    }
    #[test]
    fn expand_combo_2() {
        assert_eq!(
            "v<<A>>^AAAvA^A<vA<AA>>^AvAA<^A>Av<<A>A>^AAAvA<^A>A<vA>^A<A>A",
            Pad::expand_seq_num_then_dir_n(
                &NumSeq::new("980A"),
                2
            )
        );
    }
    #[test]
    fn expand_combo_3() {
        assert_eq!(
            "v<<A>>^A<vA<A>>^AAvAA<^A>Av<<A>>^AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A",
            Pad::expand_seq_num_then_dir_n(
                &NumSeq::new("179A"),
                2
            )
        );
    }
    #[test]
    fn expand_combo_4() {
        assert_eq!(
            "v<<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>Av<<A>A>^AAvA<^A>A",
            Pad::expand_seq_num_then_dir_n(
                &NumSeq::new("456A"),
                2
            )
        );
    }
    #[test]
    fn expand_combo_5() {
        assert_eq!(
            "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A",
            Pad::expand_seq_num_then_dir_n(
                &NumSeq::new("379A"),
                2
            )
        );
    }
    #[test]
    fn expand_combo_1_len() {
        assert_eq!(68, Pad::expand_seq_num_then_dir_n(&NumSeq::new("029A"), 2).len());
    }
    #[test]
    fn expand_combo_2_len() {
        assert_eq!(60, Pad::expand_seq_num_then_dir_n(&NumSeq::new("980A"), 2).len());
    }
    #[test]
    fn expand_combo_3_len() {
        assert_eq!(68, Pad::expand_seq_num_then_dir_n(&NumSeq::new("179A"), 2).len());
    }
    #[test]
    fn expand_combo_4_len() {
        assert_eq!(64, Pad::expand_seq_num_then_dir_n(&NumSeq::new("456A"), 2).len());
    }
    #[test]
    fn expand_combo_5_len() {
        assert_eq!(64, Pad::expand_seq_num_then_dir_n(&NumSeq::new("379A"), 2).len());
    }
    #[test]
    fn get_code_complexity_1() {
        assert_eq!(68 * 29, Pad::get_code_complexity(&NumSeq::new("029A")));
    }
    #[test]
    fn get_code_complexity_2() {
        assert_eq!(60 * 980, Pad::get_code_complexity(&NumSeq::new("980A")));
    }
    #[test]
    fn get_code_complexity_3() {
        assert_eq!(68 * 179, Pad::get_code_complexity(&NumSeq::new("179A")));
    }
    #[test]
    fn get_code_complexity_4() {
        assert_eq!(64 * 456, Pad::get_code_complexity(&NumSeq::new("456A")));
    }
    #[test]
    fn get_code_complexity_5() {
        assert_eq!(64 * 379, Pad::get_code_complexity(&NumSeq::new("379A")));
    }
    #[test]
    fn get_codes_complexity_e1() {
        assert_eq!(68 * 29 + 60 * 980 + 68 * 179 + 64 * 456 + 64 * 379, 126384);
        assert_eq!(68 * 29 + 60 * 980 + 68 * 179 + 64 * 456 + 64 * 379, Pad::get_codes_complexity(e1()));
    }
    #[test]
    fn parse_numseq_1() {
        assert_eq!(780, NumSeq::new("780A").parse_to_num());
    }
    #[test]
    fn parse_numseq_2() {
        assert_eq!(1, NumSeq::new("001A").parse_to_num());
    }
    #[test]
    fn parse_numseq_3() {
        assert_eq!(10, NumSeq::new("010A").parse_to_num());
    }
}
