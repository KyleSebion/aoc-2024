#![allow(dead_code)]

fn e1() -> &'static str {
    "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"
}
fn e1_p1_fastest() -> usize { 84 }
fn d() -> &'static str { include_str!("input.txt") }

const CHEAT_SIZE: usize = 2;

fn main() {
    for l in d().lines() {
        println!("A {l} A");
    }
}

#[cfg(test)]
mod test {
    #[test] fn a() {}
}