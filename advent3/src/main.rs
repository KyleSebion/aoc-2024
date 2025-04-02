fn get_data() -> &'static str {
    include_str!("input.txt")
}
use regex::Regex;
fn p2_get_ops_vec(d: &str) -> Vec<(String, String, String)> {
    let mut ops_enabled = true;
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))()|(don't\(\))()")
        .unwrap()
        .captures_iter(d)
        .map(|c| {
            let (f, [o1, o2]) = c.extract();
            (f.to_string(), o1.to_string(), o2.to_string())
        })
        .filter(|v| {
            match v.1.as_str() {
                "do()" => { ops_enabled = true; false }
                "don't()" => { ops_enabled = false; false }
                _ => ops_enabled
            }
        })
        .collect::<Vec<_>>()
}
fn p1_get_ops_vec(d: &str) -> Vec<(String, String, String)> {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(d)
        .map(|c| {
            let (f, [o1, o2]) = c.extract();
            (f.to_string(), o1.to_string(), o2.to_string())
        })
        .collect::<Vec<_>>()
}
fn p2_sum_ops(d: &str) -> i32 {
    p2_get_ops_vec(d)
        .iter()
        .map(|v| v.1.parse::<i32>().unwrap() * v.2.parse::<i32>().unwrap())
        .sum()
}
fn p1_sum_ops(d: &str) -> i32 {
    p1_get_ops_vec(d)
        .iter()
        .map(|v| v.1.parse::<i32>().unwrap() * v.2.parse::<i32>().unwrap())
        .sum()
}
fn main() {
    println!("{}", p1_sum_ops(get_data()));
    println!("{}", p2_sum_ops(get_data()));
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_P1: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_P2: &str = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test] fn test_p1_get_ops_vec() { assert_eq!(p1_get_ops_vec(TEST_P1).iter().map(|v| v.0.clone()).collect::<Vec<_>>(), vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]); }
    #[test] fn test_p1_sum_ops() { assert_eq!(p1_sum_ops(TEST_P1), 161); }
    #[test] fn test_p1 () { assert_eq!(p1_sum_ops(get_data()), 159892596); }
    #[test] fn test_p2_get_ops_vec() { assert_eq!(p2_get_ops_vec(TEST_P2).iter().map(|v| v.0.clone()).collect::<Vec<_>>(), vec!["mul(2,4)", "mul(8,5)"]); }
    #[test] fn test_p2_sum_ops() { assert_eq!(p2_sum_ops(TEST_P2), 48); }
    #[test] fn test_p2 () { assert_eq!(p2_sum_ops(get_data()), 92626942); }
}
