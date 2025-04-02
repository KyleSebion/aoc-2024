fn get_data() -> &'static str {
    include_str!("input.txt")
}
const MIN_CHANGE: i32 = 1;
const MAX_CHANGE: i32 = 3;
#[derive(PartialEq, Debug)]
enum Report {
    IsSafe,
    IsUnsafe,
}
fn parse_line(l: &str) -> Vec<i32> {
    l
        .split_ascii_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}
fn get_dampened_combos(v: Vec<i32>) -> Vec<Vec<i32>> {
    let mut r = Vec::new();
    for i in 0..v.len() {
        r.push(v.iter().enumerate().filter(|v| v.0 != i).map(|v| *v.1).collect::<Vec<_>>());
    }
    r.push(v);
    r
}
fn is_safe_with_dampener(l: &str) -> Report {
    let n = parse_line(l);
    let c = get_dampened_combos(n);
    let s = c.iter().map(|v| is_safe_slice(v)).any(|v| v == Report::IsSafe);
    if s { Report::IsSafe } else { Report::IsUnsafe }
}
fn is_safe_slice(n: &[i32]) -> Report { 
    let v = n.iter().zip(n.iter().skip(1));
    let mut dir = 0;
    for i in v {
        let diff = i.0 - i.1;
        let ldir = diff.signum();
        let abs = diff * ldir;
        if ldir == 0 { return Report::IsUnsafe; }
        if dir == 0 { dir = ldir; }
        if dir != ldir { return Report::IsUnsafe; }
        if !(MIN_CHANGE..=MAX_CHANGE).contains(&abs) { return Report::IsUnsafe; }
    }
    Report::IsSafe
}
fn is_safe_str(l: &str) -> Report {
    is_safe_slice(&parse_line(l))
}
fn part_1_count_of_safe_reports(d: &str) -> usize {
    d.trim().lines().map(is_safe_str).filter(|v| *v == Report::IsSafe).count()
}
fn part_2_count_of_safe_reports(d: &str) -> usize {
    d.trim().lines().map(is_safe_with_dampener).filter(|v| *v == Report::IsSafe).count()
}
fn main() {
    println!("{}", part_1_count_of_safe_reports(get_data())); // 524
    println!("{}", part_2_count_of_safe_reports(get_data())); // 569
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn p1() { assert_eq!(part_1_count_of_safe_reports(get_data()), 524); }
    #[test] fn p2() { assert_eq!(part_2_count_of_safe_reports(get_data()), 569); }
    #[test] fn test_is_safe_with_dampener_1() { assert_eq!(is_safe_with_dampener("7 6 4 2 1"), Report::IsSafe);   }
    #[test] fn test_is_safe_with_dampener_2() { assert_eq!(is_safe_with_dampener("1 2 7 8 9"), Report::IsUnsafe); }
    #[test] fn test_is_safe_with_dampener_3() { assert_eq!(is_safe_with_dampener("9 7 6 2 1"), Report::IsUnsafe); }
    #[test] fn test_is_safe_with_dampener_4() { assert_eq!(is_safe_with_dampener("1 3 2 4 5"), Report::IsSafe);   }
    #[test] fn test_is_safe_with_dampener_5() { assert_eq!(is_safe_with_dampener("8 6 4 4 1"), Report::IsSafe);   }
    #[test] fn test_is_safe_with_dampener_6() { assert_eq!(is_safe_with_dampener("1 3 6 7 9"), Report::IsSafe);   }
    #[test] fn test_is_safe_str_1() { assert_eq!(is_safe_str("7 6 4 2 1"), Report::IsSafe);   }
    #[test] fn test_is_safe_str_2() { assert_eq!(is_safe_str("1 2 7 8 9"), Report::IsUnsafe); }
    #[test] fn test_is_safe_str_3() { assert_eq!(is_safe_str("9 7 6 2 1"), Report::IsUnsafe); }
    #[test] fn test_is_safe_str_4() { assert_eq!(is_safe_str("1 3 2 4 5"), Report::IsUnsafe); }
    #[test] fn test_is_safe_str_5() { assert_eq!(is_safe_str("8 6 4 4 1"), Report::IsUnsafe); }
    #[test] fn test_is_safe_str_6() { assert_eq!(is_safe_str("1 3 6 7 9"), Report::IsSafe);   }
    #[test]
    fn test_part_1_count_of_safe_reports() {
        let d = "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        ";
        assert_eq!(part_1_count_of_safe_reports(d), 2);
    }
}
