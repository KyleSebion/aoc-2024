#![allow(dead_code)]

fn d() -> &'static str {
    include_str!("input.txt")
}
fn e1() -> &'static str {
    "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]#[rustfmt::skip]
mod test {
    use super::*;
    #[test] fn a() {}
}
