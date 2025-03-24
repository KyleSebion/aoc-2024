#![allow(dead_code, clippy::let_and_return)]
const fn d() -> &'static str {
    include_str!("input.txt")
}
const fn e1() -> &'static str {
    "\
1
10
100
2024
"
}
const fn e2() -> &'static str {
    "\
1
2
3
2024
"
}

fn get_next_secret_verbose(s: usize) -> usize {
    let s1 = s * 64;
    let s = s ^ s1;
    let s = s % 16777216;
    let s2 = s / 32;
    let s = s ^ s2;
    let s = s % 16777216;
    let s3 = s * 2048;
    let s = s ^ s3;
    let s = s % 16777216;
    s
}
fn get_next_secret_terse(s0: usize) -> usize {
    let s1 = (16777216 - 1) & (s0 ^ (s0 << 6));
    let s2 = (16777216 - 1) & (s1 ^ (s1 >> 5));
    let s3 = (16777216 - 1) & (s2 ^ (s2 << 11));
    s3
}
fn get_next_secret(s: usize) -> usize {
    get_next_secret_terse(s)
}
fn get_next_secret_nth(s: usize, nth: usize) -> usize {
    (0..nth).fold(s, |s, _| get_next_secret(s))
}
fn get_next_secrets_nth(d: &str, nth: usize) -> Vec<usize> {
    d.lines()
        .map(|s| get_next_secret_nth(s.parse::<usize>().unwrap(), nth))
        .collect()
}
fn get_next_secrets_nth_sum(d: &str, nth: usize) -> usize {
    get_next_secrets_nth(d, nth).into_iter().sum()
}
fn main() {
    println!("{:?}", get_next_secrets_nth_sum(d(), 2000));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t_123() {
        let mut start = 123;
        let rs = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        for next in rs {
            let r1 = get_next_secret_terse(start);
            let r2 = get_next_secret_verbose(start);
            assert_eq!(r1, r2);
            assert_eq!(r1, next);
            start = next;
        }
    }
    #[test]
    fn t_123_nth10() {
        assert_eq!(5908254, get_next_secret_nth(123, 10))
    }
    #[test]
    fn t_e1_each() {
        assert_eq!(
            vec![8685429, 4700978, 15273692, 8667524],
            get_next_secrets_nth(e1(), 2000)
        )
    }
    #[test]
    fn t_e1_sum() {
        assert_eq!(37327623, get_next_secrets_nth_sum(e1(), 2000))
    }
}
