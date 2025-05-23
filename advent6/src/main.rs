use std::{
    sync::Mutex,
    thread,
    time::{Duration, Instant},
};

fn get_p1_test_data() -> &'static str {
    "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
}
fn get_p1_data() -> &'static str {
    include_str!("input.txt")
}
struct MapSpace {
    space: char,
    guard: char,
    visited: bool,
    visited_up: bool,
    visited_down: bool,
    visited_left: bool,
    visited_right: bool,
}
impl MapSpace {
    fn new(v: char) -> MapSpace {
        let mut s = MapSpace {
            space: '.',
            guard: ' ',
            visited: false,
            visited_up: false,
            visited_down: false,
            visited_left: false,
            visited_right: false,
        };
        match v {
            '#' => s.space = v,
            '>' | '<' | '^' | 'v' | 'V' => {
                s.guard = v;
                s.visited = true;
            }
            _ => {}
        }
        s
    }
    fn is_obstruction(&self) -> bool {
        self.space == '#'
    }
    fn was_visited(&self) -> bool {
        self.visited
    }
    fn is_guard_val(v: char) -> bool {
        matches!(v, '>' | '<' | '^' | 'v' | 'V')
    }
    fn is_guard(&self) -> bool {
        Self::is_guard_val(self.guard)
    }
    fn get_guard_dir(&self) -> Option<(usize, usize)> {
        match self.guard {
            '>' => Some((1, 0)),
            '<' => Some((usize::MAX, 0)),
            '^' => Some((0, usize::MAX)),
            'v' | 'V' => Some((0, 1)),
            _ => None,
        }
    }
    fn get_normal(&self) -> char {
        if self.is_guard() {
            self.guard
        } else {
            self.space
        }
    }
    fn get_visited(&self) -> char {
        if self.was_visited() {
            'X'
        } else {
            self.get_normal()
        }
    }
    fn clear_guard(&mut self) {
        self.guard = ' ';
    }
    fn set_guard(&mut self, v: char) -> bool {
        self.guard = v;
        self.visited = true;
        let vis = match self.guard {
            '>' => &mut self.visited_right,
            '<' => &mut self.visited_left,
            '^' => &mut self.visited_up,
            'v' | 'V' => &mut self.visited_down,
            _ => panic!("bad guard {v}!"),
        };
        let r = *vis;
        *vis = true;
        r
    }
    fn rotate_guard(&mut self) {
        match self.guard {
            '>' => self.guard = 'v',
            '<' => self.guard = '^',
            '^' => self.guard = '>',
            'v' => self.guard = '<',
            'V' => self.guard = '<',
            _ => {}
        }
    }
}
struct Map {
    rows: usize,
    cols: usize,
    map: Vec<MapSpace>,
    guard_i: usize,
}
impl Map {
    const GUARD_UNK: usize = usize::MAX;
    const GUARD_DONE: usize = usize::MAX - 1;
    fn new(d: &str) -> Map {
        let mut m = Map {
            rows: 0,
            cols: 0,
            map: Vec::new(),
            guard_i: Self::GUARD_UNK,
        };
        for l in d.lines() {
            m.rows += 1;
            let mut col_count = 0;
            for c in l.chars() {
                col_count += 1;
                m.map.push(MapSpace::new(c));
            }
            if m.cols == 0 {
                m.cols = col_count;
            }
            if m.cols != col_count {
                panic!(
                    "column counts vary between row {} and row {}",
                    m.rows - 1,
                    m.rows
                );
            }
        }
        m
    }
    fn i_to_xy(&self, i: usize) -> (usize, usize) {
        let x = i % self.cols;
        let y = i / self.cols;
        (x, y)
    }
    fn xy_to_i(&self, (x, y): (usize, usize)) -> usize {
        y * self.cols + x
    }
    fn iterate_xy(&self, f: fn(&MapSpace) -> char) -> String {
        let mut s = String::new();
        for y in 0..self.rows {
            for x in 0..self.cols {
                let i = self.xy_to_i((x, y));
                let ms = &self.map[i];
                s.push(f(ms));
            }
            s.push('\n');
        }
        s
    }
    fn get_normal(&self) -> String {
        self.iterate_xy(|ms| ms.get_normal())
    }
    fn get_visited(&self) -> String {
        self.iterate_xy(|ms| ms.get_visited())
    }
    fn guard_pos(&mut self) -> Option<usize> {
        if self.guard_i == Self::GUARD_DONE {
            None
        } else {
            if self.guard_i == Self::GUARD_UNK {
                self.guard_i = self.map.iter().position(|ms| ms.is_guard())?;
            }
            Some(self.guard_i)
        }
    }
    fn is_valid_pos(&self, (x, y): (usize, usize)) -> bool {
        (0..self.cols).contains(&x) && (0..self.rows).contains(&y)
    }
    fn move_guard(&mut self) -> (bool, bool) {
        if let Some(cur_i) = self.guard_pos() {
            let mut loop_detected = false;
            let (cur_x, cur_y) = self.i_to_xy(cur_i);
            let cur_g = &self.map[cur_i];
            let (dx, dy) = cur_g.get_guard_dir().unwrap();
            let (new_x, new_y) = (cur_x.wrapping_add(dx), cur_y.wrapping_add(dy));
            let char_g = cur_g.guard;
            if self.is_valid_pos((new_x, new_y)) {
                let new_i = self.xy_to_i((new_x, new_y));
                if self.map[new_i].is_obstruction() {
                    self.map[cur_i].rotate_guard();
                } else {
                    self.map[cur_i].clear_guard();
                    loop_detected = self.map[new_i].set_guard(char_g);
                    self.guard_i = new_i;
                }
            } else {
                self.map[cur_i].clear_guard();
                self.guard_i = Self::GUARD_DONE;
            }
            return (true, loop_detected);
        }
        (false, false)
    }
    fn get_count_visited(&self) -> usize {
        self.get_visited().chars().filter(|&c| c == 'X').count()
    }
}
fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    //print!("{esc}[2J", esc = 27 as char);
}
fn animated_move(m: &mut Map) {
    print!("{}", m.get_normal());
    while m.move_guard().0 {
        std::thread::sleep(Duration::from_millis(10));
        clear_screen();
        print!("{}", m.get_normal());
    }
}
fn unanimated_move(m: &mut Map) {
    while m.move_guard().0 {}
}
fn p1(l: usize) {
    let _x = get_p1_test_data();
    let s = Instant::now();
    let mut m = Map::new(get_p1_data());
    if l > 1 {
        animated_move(&mut m);
    } else {
        unanimated_move(&mut m);
    }
    println!("{}x{} {}", m.cols, m.rows, m.get_count_visited());
    println!("{:?}", s.elapsed());
}
#[allow(dead_code)]
fn find_loops(d: &str) -> usize {
    let mut loops = 0;
    for i in 0..Map::new(d).map.len() {
        let mut m = Map::new(d);
        if m.map[i].space == '.' {
            m.map[i] = MapSpace::new('#');
            loop {
                let (is_done, loop_detected) = m.move_guard();
                if loop_detected {
                    loops += 1;
                    break;
                }
                if !is_done {
                    break;
                }
            }
        }
    }
    loops
}
fn print_thread(se: char, tn: usize, procs: &usize, m: &Mutex<usize>) {
    print!("{se}{tn} ");
    let mut g = m.lock().unwrap();
    *g += 1;
    if *g == *procs {
        *g = 0;
        println!(" ");
    }
}
fn find_loops_mt(d: &str) -> isize {
    let m = &Mutex::new(0);
    let procs = &std::thread::available_parallelism().unwrap().get();
    let map_len = &Map::new(d).map.len();
    let map_span = &(map_len / procs);
    thread::scope(|s| {
        (0..*procs)
            .map(|tn| {
                s.spawn(move || {
                    print_thread('S', tn, procs, m);
                    let mut loops = 0;
                    for i in (tn * map_span)..(tn * map_span + map_span) {
                        let mut m = Map::new(d);
                        if m.map[i].space == '.' {
                            m.map[i] = MapSpace::new('#');
                            loop {
                                let (is_done, loop_detected) = m.move_guard();
                                if loop_detected {
                                    loops += 1;
                                    break;
                                }
                                if !is_done {
                                    break;
                                }
                            }
                        }
                    }
                    print_thread('E', tn, procs, m);
                    loops
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|jh| jh.join().unwrap())
            .sum()
    })
}
fn p2(l: usize) {
    if l > 1 {
        println!("no args needed");
    } else {
        let s = Instant::now();
        let loops = find_loops_mt(get_p1_data());
        println!("loops: {loops}");
        println!("{:?}", s.elapsed());
    }
}
fn main() {
    p1(std::env::args().collect::<Vec<_>>().len());
    p2(std::env::args().collect::<Vec<_>>().len());
}
#[cfg(test)]
pub mod tests {
    use super::*;
    fn get_test_map() -> Map {
        Map::new(get_p1_test_data())
    }
    #[test]
    fn test_xy_to_i_on_test_data() {
        let m = get_test_map();
        assert_eq!(11, m.xy_to_i((1, 1)));
        assert_eq!(80, m.xy_to_i((0, 8)));
        assert_eq!(29, m.xy_to_i((9, 2)));
    }
    #[test]
    fn test_i_to_xy_on_test_data() {
        let m = get_test_map();
        assert_eq!((1, 1), m.i_to_xy(11));
        assert_eq!((0, 8), m.i_to_xy(80));
        assert_eq!((9, 2), m.i_to_xy(29));
    }
    #[test]
    fn test_p1_test_data() {
        let mut m = get_test_map();
        while m.move_guard().0 {}
        assert_eq!(41, m.get_count_visited());
    }
    #[test]
    fn test_p1() {
        let mut m = Map::new(get_p1_data());
        while m.move_guard().0 {}
        assert_eq!(4973, m.get_count_visited());
    }
    #[test]
    fn test_p2_test_data() {
        assert_eq!(6, find_loops(get_p1_test_data()));
    }
    #[test]
    fn test_p2_data() {
        assert_eq!(1482, find_loops_mt(get_p1_data()));
    }
}
