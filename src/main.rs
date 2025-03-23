#![allow(dead_code)]

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashMap;
use std::time::Instant;

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
    v: &'a str,
}
impl<'a> NumSeq<'a> {
    fn new(seq: &'a str) -> Self {
        if seq
            .chars()
            .all(|c| ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'].contains(&c))
        {
            Self { v: seq }
        } else {
            panic!("invalid NumSeq: {seq}");
        }
    }
    fn parse_to_num(&self) -> usize {
        self.v
            .trim_start_matches('0')
            .trim_end_matches('A')
            .parse()
            .expect("parse_to_num failed")
    }
}
struct DirSeq<'a> {
    v: &'a str,
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
                pad: vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']],
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
        // self.x = to_x; // use self.reduce_seq_val instead to trigger oob check
        // self.y = to_y;
        let seq = if goes_oob { dy + &dx } else { dx + &dy } + "A";
        self.reduce_seq_val(&DirSeq::new(&seq)); // instead of changing self.x and self.y directly so that oob can be detected
        seq
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
    fn expand_multi_seq(seqs: &[&str]) -> String {
        seqs.iter()
            .map(|&seq| Self::expand_seq_dir(&DirSeq::new(seq)))
            .join(" ")
    }
    fn validate(&self) {
        if self.x >= self.pad[0].len() {
            panic!("bad x: {}", self.x);
        }
        if self.y >= self.pad.len() {
            panic!("bad y: {}", self.y);
        }
        if self.get_pad_val() == ' ' {
            panic!("oob");
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
        seq.v
            .split_terminator('A')
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
        codes
            .lines()
            .map(|code| Self::get_code_complexity(&NumSeq::new(code)))
            .sum()
    }
}

fn get_dir_variations(v: char, h: char) -> Vec<String> {
    (1..)
        .fold_while(Vec::new(), |mut acc, size| {
            let vari = get_dir_variations_at_size(v, h, size);
            if vari.is_empty() {
                Done(acc)
            } else {
                acc.extend(vari);
                Continue(acc)
            }
        })
        .into_inner()
}
fn get_dir_variations_at_size(v: char, h: char, size: usize) -> Vec<String> {
    let pos = [v, h, h];
    pos.into_iter()
        .permutations(size)
        .unique()
        .map(|c| c.into_iter().collect::<String>() + "A")
        .collect()
}
fn get_dir_variations_at_sizes(v: char, h: char, v_size: usize, h_size: usize) -> Vec<String> {
    let mut pos = vec![v; v_size];
    pos.extend(vec![h; h_size]);
    pos.into_iter()
        .permutations(h_size + v_size)
        .unique()
        .map(|c| c.into_iter().collect::<String>() + "A")
        .collect()
}

fn get_smallest_strings(vs: Vec<String>) -> Vec<String> {
    // let min = vs.iter().map(|s| s.len()).min().unwrap();
    // vs.into_iter().filter(|s| s.len() == min).collect_vec()
    vs
}
fn merge_combos(mut vs: Vec<Vec<String>>) -> Vec<String> {
    let mut nv = vec!["".to_owned()];
    while let Some(vp) = vs.pop() {
        let mut nnv = Vec::new();
        for ns in vp {
            for os in &nv {
                nnv.push(format!("{ns}{os}"));
            }
        }
        nv = nnv;
    }
    nv
}
fn code_to_dirs(code: &str) -> Vec<String> {
    let mut r = Pad::new(PadType::Num);
    get_smallest_strings(merge_combos(
        code.chars().map(|c| char_to_dirs(c, &mut r)).collect(),
    ))
}
fn dirs_to_dirs(dirs: Vec<String>) -> Vec<String> {
    get_smallest_strings(
        dirs.into_iter()
            .flat_map(|dir| dir_to_dirs(&dir))
            .collect_vec(),
    )
}
fn dir_to_dirs(dir: &str) -> Vec<String> {
    let mut r = Pad::new(PadType::Dir);
    get_smallest_strings(merge_combos(
        dir.chars().map(|c| char_to_dirs(c, &mut r)).collect(),
    ))
}
fn char_to_dirs(c: char, r: &mut Pad) -> Vec<String> {
    let (to_x, to_y) = r.hm[&c];
    let h = if r.x > to_x { '<' } else { '>' };
    let v = if r.y > to_y { '^' } else { 'v' };
    let dx = to_x.abs_diff(r.x);
    let dy = to_y.abs_diff(r.y);
    let vs = get_dir_variations_at_sizes(v, h, dy, dx)
        .into_iter()
        .filter(|seq| {
            let mut x = r.x;
            let mut y = r.y;
            let sp = r.hm[&' '];
            for c in seq.chars() {
                match c {
                    '>' => x += 1,
                    '<' => x -= 1,
                    '^' => y -= 1,
                    'v' => y += 1,
                    'A' => {}
                    _ => panic!("no"),
                }
                if (x, y) == sp {
                    return false;
                }
            }
            true
        })
        .collect_vec();
    if vs.is_empty() {
        panic!("vs.is_empty");
    }
    r.x = to_x;
    r.y = to_y;
    get_smallest_strings(vs)
}
fn get_min_len_code_dir(code: &str) -> usize {
    dirs_to_dirs(dirs_to_dirs(code_to_dirs(code)))
        .into_iter()
        .map(|dir| dir.len())
        .min()
        .expect("get_min_len_code_dir")
}
fn get_code_complexity2(code: &str) -> usize {
    get_min_len_code_dir(code) * NumSeq::new(code).parse_to_num()
}
fn get_codes_complexity2(codes: &str) -> usize {
    codes.lines().map(get_code_complexity2).sum()
}
fn print_movements() {
    let mut bt = std::collections::BTreeMap::new();
    for start in "<>^vA".chars() {
        for end in "<>^vA".chars() {
            let mut r = Pad::new(PadType::Dir);
            let (x, y) = r.hm[&start];
            r.x = x;
            r.y = y;
            let mut res = char_to_dirs(end, &mut r);
            res.sort();
            bt.insert((start, end), res);
        }
    }
    for (k, v) in bt {
        println!("{k:?} {v:?}");
    }

    let mut bt = std::collections::BTreeMap::new();
    for start in "01234567890A".chars() {
        for end in "01234567890A".chars() {
            let mut r = Pad::new(PadType::Num);
            let (x, y) = r.hm[&start];
            r.x = x;
            r.y = y;
            let mut res = char_to_dirs(end, &mut r);
            res.sort();
            bt.insert((start, end), res);
        }
    }
    for (k, v) in bt {
        println!("({k:?}, vec!{v:?}),");
    }
    
    const WIDTH: usize = 9;
    for vi in [">vA", "v>A"] {
        let mut r = Pad::new(PadType::Dir);
        print!("{:^WIDTH$}", format!("A{vi}"));
        for mut vvi in vi.chars().map(|c| char_to_dirs(c, &mut r)) {
            vvi.sort();
            print!("{:^WIDTH$}", vvi.join("|"));
        }
        println!();
    }
} 
fn main() {
    // let s = Instant::now();
    // println!("START");
    // get_codes_complexity2(d());
    // println!("{:?} END", s.elapsed());

    print_movements();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn reduce_dir_pad_1() {
        assert_eq!(
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A",
            Pad::new(PadType::Dir).reduce_seq_val(&DirSeq::new(
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
            ))
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
        assert_eq!(
            "029A",
            Pad::new(PadType::Num).reduce_seq_val(&DirSeq::new("<A^A>^^AvvvA"))
        );
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
                &DirSeq::new(
                    "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
                ),
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
                &DirSeq::new(
                    "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
                ),
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
            Pad::expand_seq_num_then_dir_n(&NumSeq::new("029A"), 2)
        );
    }
    #[test]
    fn expand_combo_2() {
        assert_eq!(
            "v<<A>>^AAAvA^A<vA<AA>>^AvAA<^A>Av<<A>A>^AAAvA<^A>A<vA>^A<A>A",
            Pad::expand_seq_num_then_dir_n(&NumSeq::new("980A"), 2)
        );
    }
    #[test]
    fn expand_combo_3() {
        assert_eq!(
            "v<<A>>^A<vA<A>>^AAvAA<^A>Av<<A>>^AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A",
            Pad::expand_seq_num_then_dir_n(&NumSeq::new("179A"), 2)
        );
    }
    #[test]
    fn expand_combo_4() {
        assert_eq!(
            "v<<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>Av<<A>A>^AAvA<^A>A",
            Pad::expand_seq_num_then_dir_n(&NumSeq::new("456A"), 2)
        );
    }
    #[test]
    fn expand_combo_5() {
        assert_eq!(
            "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A",
            Pad::expand_seq_num_then_dir_n(&NumSeq::new("379A"), 2)
        );
    }
    #[test]
    fn expand_combo_1_len() {
        assert_eq!(
            68,
            Pad::expand_seq_num_then_dir_n(&NumSeq::new("029A"), 2).len()
        );
    }
    #[test]
    fn expand_combo_2_len() {
        assert_eq!(
            60,
            Pad::expand_seq_num_then_dir_n(&NumSeq::new("980A"), 2).len()
        );
    }
    #[test]
    fn expand_combo_3_len() {
        assert_eq!(
            68,
            Pad::expand_seq_num_then_dir_n(&NumSeq::new("179A"), 2).len()
        );
    }
    #[test]
    fn expand_combo_4_len() {
        assert_eq!(
            64,
            Pad::expand_seq_num_then_dir_n(&NumSeq::new("456A"), 2).len()
        );
    }
    #[test]
    fn expand_combo_5_len() {
        assert_eq!(
            64,
            Pad::expand_seq_num_then_dir_n(&NumSeq::new("379A"), 2).len()
        );
    }
    #[test]
    fn code_1_len() {
        assert_eq!(68, get_min_len_code_dir("029A"));
    }
    #[test]
    fn code_2_len() {
        assert_eq!(60, get_min_len_code_dir("980A"));
    }
    #[test]
    fn code_3_len() {
        assert_eq!(68, get_min_len_code_dir("179A"));
    }
    #[test]
    fn code_4_len() {
        assert_eq!(64, get_min_len_code_dir("456A"));
    }
    #[test]
    fn code_5_len() {
        assert_eq!(64, get_min_len_code_dir("379A"));
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
        assert_eq!(
            68 * 29 + 60 * 980 + 68 * 179 + 64 * 456 + 64 * 379,
            Pad::get_codes_complexity(e1())
        );
    }
    #[test]
    fn get_codes_complexity2_e1() {
        assert_eq!(68 * 29 + 60 * 980 + 68 * 179 + 64 * 456 + 64 * 379, 126384);
        assert_eq!(
            68 * 29 + 60 * 980 + 68 * 179 + 64 * 456 + 64 * 379,
            get_codes_complexity2(e1())
        );
    }
    #[test]
    fn get_codes_complexity2_d() {
        assert_eq!(174124, get_codes_complexity2(d()));
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
