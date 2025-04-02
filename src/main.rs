use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};
#[allow(dead_code)]
fn get_ex_antis() -> &'static str {
    "\
......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
"
}
#[allow(dead_code)]
fn get_ex() -> &'static str {
    "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
}
#[allow(dead_code)]
fn get_ex2() -> &'static str {
    "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
"
}
#[allow(dead_code)]
fn get_ex2_antis() -> &'static str {
    "\
T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........
"
}
fn get_data() -> &'static str {
    include_str!("input.txt")
}
#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}
struct MapSpace {
    kind: char,
    antinodes: HashSet<char>,
}
impl MapSpace {
    fn is_freq(&self) -> bool {
        self.kind != '.'
    }
}
struct Map {
    board: Vec<Vec<MapSpace>>,
    freqs: HashMap<char, Vec<Point>>,
}
impl Map {
    fn new(d: &str, part1: bool) -> Self {
        let mut m = Map {
            board: vec![],
            freqs: HashMap::new(),
        };
        for l in d.lines() {
            let mut row = vec![];
            for c in l.chars() {
                row.push(MapSpace {
                    kind: c,
                    antinodes: HashSet::new(),
                });
            }
            m.board.push(row);
        }
        if m.board
            .iter()
            .map(|v| v.len())
            .collect::<HashSet<_>>()
            .len()
            != 1
        {
            panic!("Row width mismatch");
        }
        m.cache_freqs();
        if part1 {
            m.render_antinodes_part1();
        } else {
            m.render_antinodes_part2();
        }
        m
    }
    fn cache_freqs(&mut self) {
        for (y, row) in self.board.iter().enumerate() {
            for (x, space) in row.iter().enumerate() {
                if space.is_freq() {
                    let p = Point { x, y };
                    self.freqs.entry(space.kind).or_default().push(p);
                }
            }
        }
    }
    fn is_in_bounds(&self, p: &Point) -> bool {
        p.x < self.board[0].len() && p.y < self.board.len()
    }
    fn render_antinodes_part1(&mut self) {
        for (freq, ps) in self.freqs.iter() {
            for (i, p1) in ps.iter().enumerate() {
                for p2 in ps.iter().skip(i + 1) {
                    let antinodes = self.get_valid_antinode_points_part1(p1, p2);
                    for ap in antinodes {
                        self.board[ap.y][ap.x].antinodes.insert(*freq);
                    }
                }
            }
        }
    }
    fn get_valid_antinode_points_part1(&self, p1: &Point, p2: &Point) -> Vec<Point> {
        let mut v = vec![];
        if let Some(p) = self.get_valid_antinode_point_part1(p1, p2) {
            v.push(p);
        }
        if let Some(p) = self.get_valid_antinode_point_part1(p2, p1) {
            v.push(p);
        }
        v
    }
    fn get_valid_antinode_point_part1(&self, p1: &Point, p2: &Point) -> Option<Point> {
        let dx = p1.x.wrapping_sub(p2.x);
        let dy = p1.y.wrapping_sub(p2.y);
        let ap = Point {
            x: p1.x.wrapping_add(dx),
            y: p1.y.wrapping_add(dy),
        };
        if self.is_in_bounds(&ap) {
            Some(ap)
        } else {
            None
        }
    }
    fn render_antinodes_part2(&mut self) {
        for (freq, ps) in self.freqs.iter() {
            for (i, p1) in ps.iter().enumerate() {
                for p2 in ps.iter().skip(i + 1) {
                    let antinodes = self.get_valid_antinode_points_part2(p1, p2);
                    for ap in antinodes {
                        self.board[ap.y][ap.x].antinodes.insert(*freq);
                    }
                }
            }
        }
    }
    fn get_valid_antinode_points_part2(&self, p1: &Point, p2: &Point) -> Vec<Point> {
        let mut v = self.get_valid_antinode_point_part2(p1, p2);
        v.extend(self.get_valid_antinode_point_part2(p2, p1));
        v
    }
    fn get_valid_antinode_point_part2(&self, p1: &Point, p2: &Point) -> Vec<Point> {
        let dx = p1.x.wrapping_sub(p2.x);
        let dy = p1.y.wrapping_sub(p2.y);
        let mut v = vec![*p1];
        let mut last_ap = *p1;
        loop {
            let ap = Point {
                x: last_ap.x.wrapping_add(dx),
                y: last_ap.y.wrapping_add(dy),
            };
            if self.is_in_bounds(&ap) {
                v.push(ap);
                last_ap = ap;
            } else {
                break;
            }
        }
        v
    }
    #[allow(dead_code)]
    fn get_map_freqs(&self) -> String {
        let mut s = String::new();
        for row in self.board.iter() {
            for space in row.iter() {
                s.push(space.kind);
            }
            s.push('\n');
        }
        s
    }
    #[allow(dead_code)]
    fn get_map_antis(&self) -> String {
        let mut s = String::new();
        for row in self.board.iter() {
            for space in row.iter() {
                if space.is_freq() {
                    s.push(space.kind);
                } else if space.antinodes.is_empty() {
                    s.push('.');
                } else {
                    s.push('#');
                }
            }
            s.push('\n');
        }
        s
    }
    #[allow(dead_code)]
    fn get_map_just_antis(&self, part1: bool) -> String {
        let mut s = String::new();
        for row in self.board.iter() {
            for space in row.iter() {
                if part1 {
                    if space.is_freq() || space.antinodes.is_empty() {
                        s.push('.');
                    } else {
                        s.push('#');
                    }
                } else if space.antinodes.is_empty() {
                    s.push('.');
                } else {
                    s.push('#');
                }
            }
            s.push('\n');
        }
        s
    }
    fn get_count_antis(&self) -> usize {
        self.board
            .iter()
            .flat_map(|v| v.iter())
            .filter(|s| !s.antinodes.is_empty())
            .count()
    }
}
fn main() {
    let s = Instant::now();
    println!("{} {:?}", Map::new(get_data(), false).get_count_antis(), s.elapsed());
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn p1_ex_antis() {
        assert_eq!(get_ex_antis(), Map::new(get_ex(), true).get_map_antis());
    }
    #[test]
    fn p1_ex_map() {
        assert_eq!(get_ex(), Map::new(get_ex(), true).get_map_freqs());
    }
    #[test]
    fn p1_ex() {
        assert_eq!(14, Map::new(get_ex(), true).get_count_antis());
    }
    #[test]
    fn p1() {
        assert_eq!(222, Map::new(get_data(), true).get_count_antis());
    }
    #[test]
    fn p2_ex_antis() {
        assert_eq!(get_ex2_antis(), Map::new(get_ex2(), false).get_map_antis());
    }
    #[test]
    fn p2_ex2() {
        assert_eq!(9, Map::new(get_ex2(), false).get_count_antis());
    }
    #[test]
    fn p2_ex() {
        assert_eq!(34, Map::new(get_ex(), false).get_count_antis());
    }
    #[test]
    fn p2() {
        assert_eq!(884, Map::new(get_data(), false).get_count_antis());
    }
}
