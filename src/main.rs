#![allow(dead_code)]

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

enum PadType {
    Num,
    Dir,
}
struct Pad {
    x: usize,
    y: usize,
    pad: Vec<Vec<char>>,
}
impl Pad {
    fn new(t: PadType) -> Self {
        match t {
            PadType::Dir => Self {
                pad: vec![
                    vec![' ', '^', 'A'],
                    vec!['<', 'v', '>']
                ],
                x: 2,
                y: 0,
            },
            PadType::Num => Self {
                pad: vec![
                    vec!['7', '8', '9'],
                    vec!['4', '5', '6'],
                    vec!['1', '2', '3'],
                    vec![' ', '0', 'A'],
                ],
                x: 2,
                y: 3,
            },
        }
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
    fn validate_seq(seq: &str) {
        if !seq.chars().all(|c| ['^', 'v', '>', '<', 'A'].contains(&c)) || !seq.ends_with('A') {
            panic!("invalid seq: {seq}");
        }
    }
    fn reduce_seq_val(&mut self, seq: &str) -> String {
        Self::validate_seq(seq);
        seq.split_terminator('A')
            .map(|sub| {
                for dir in sub.chars() {
                    self.reduce(dir);
                }
                self.get_pad_val()
            })
            .join("")
    }
    fn reduce_seq_dir(seq: &str) -> String {
        Self::validate_seq(seq);
        Self::new(PadType::Dir).reduce_seq_val(seq)
    }
    fn reduce_seq_num(seq: &str) -> String {
        Self::validate_seq(seq);
        Self::new(PadType::Num).reduce_seq_val(seq)
    }
    fn reduce_seq_dir_n(seq: &str, n: usize) -> String {
        Self::validate_seq(seq);
        let mut res = seq.to_owned();
        for _ in 0..n {
            res = Self::reduce_seq_dir(&res);
        }
        res
    }
    fn reduce_seq_dir_n_then_num(seq: &str, n: usize) -> String {
        Self::validate_seq(seq);
        let res = Self::reduce_seq_dir_n(seq, n);
        Self::reduce_seq_num(&res)
    }

}

fn main() {
    println!(
        "{}",
        Pad::reduce_seq_dir_n_then_num(
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
            2
        )
    );
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn reduce_dir_pad_1() {
        assert_eq!(
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A",
            Pad::new(PadType::Dir).reduce_seq_val(
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
            )
        );
    }
    #[test]
    fn reduce_dir_pad_2() {
        assert_eq!(
            "<A^A>^^AvvvA",
            Pad::new(PadType::Dir).reduce_seq_val("v<<A>>^A<A>AvA<^AA>A<vAAA>^A")
        );
    }
    #[test]
    fn reduce_dir_pad_3() {
        assert_eq!(
            "<A^A>^^AvvvA",
            Pad::reduce_seq_dir("v<<A>>^A<A>AvA<^AA>A<vAAA>^A")
        );
    }
    #[test]
    fn reduce_num_pad_1() {
        assert_eq!("029A", Pad::new(PadType::Num).reduce_seq_val("<A^A>^^AvvvA"));
    }
    #[test]
    fn reduce_num_pad_2() {
        assert_eq!("029A", Pad::reduce_seq_num("<A^A>^^AvvvA"));
    }
    #[test]
    fn reduce_combo_1() {
        assert_eq!(
            "029A",
            Pad::reduce_seq_dir_n_then_num(
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
                2
            )
        );
    }
    #[test]
    fn reduce_combo_2() {
        assert_eq!(
            "980A",
            Pad::reduce_seq_dir_n_then_num(
                "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
                2
            )
        );
    }
    #[test]
    fn reduce_combo_3() {
        assert_eq!(
            "179A",
            Pad::reduce_seq_dir_n_then_num(
                "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
                2
            )
        );
    }
    #[test]
    fn reduce_combo_4() {
        assert_eq!(
            "456A",
            Pad::reduce_seq_dir_n_then_num(
                "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
                2
            )
        );
    }
    #[test]
    fn reduce_combo_5() {
        assert_eq!(
            "379A",
            Pad::reduce_seq_dir_n_then_num(
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
                2
            )
        );
    }
}
