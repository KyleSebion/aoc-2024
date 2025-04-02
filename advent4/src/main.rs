fn get_data() -> &'static str {
    include_str!("input.txt")
}
const CROSSWORD_WORD1: &str = "XMAS";
const CROSSWORD_WORD2: &str = "MAS";
type INT = isize;
struct Crossword {
    board: Vec<char>,
    x_len: INT,
    y_len: INT,
}
impl Crossword {
    fn new(data: &str) -> Self {
        let mut board = Vec::new();
        let mut x_len = 0;
        let mut y_len = 0;
        if !data.is_ascii() { panic!("only an ascii board is supported"); }
        for l in data.lines() {
            let len = INT::try_from(l.len()).unwrap();
            if len == 0 { panic!("board length cannot be 0"); }
            if x_len == 0 { x_len = len; }
            assert_eq!(x_len, len);
            y_len += 1;
            board.extend(l.chars());
        }
        Self { board, x_len, y_len }
    }
    fn get_board_len(&self) -> INT { INT::try_from(self.board.len()).unwrap() }
    fn validate_len_contains(v: INT, len: INT) -> Option<INT> { if (0..len).contains(&v) { return Some(v); } else { return None; } }
    fn validate_i(&self, i: INT) -> Option<INT> { Self::validate_len_contains(i, self.get_board_len()) }
    fn validate_x(&self, x: INT) -> Option<INT> { Self::validate_len_contains(x, self.x_len) }
    fn validate_y(&self, y: INT) -> Option<INT> { Self::validate_len_contains(y, self.y_len) }
    fn i_to_xy(&self, i: INT) -> Option<(INT, INT)> {
        let i = self.validate_i(i)?;
        let x = self.validate_x(i % self.x_len)?;
        let y = self.validate_y(i / self.x_len)?;
        Some((x, y))
    }
    fn xy_to_i(&self, x: INT, y: INT) -> Option<INT> {
        let x = self.validate_x(x)?;
        let y = self.validate_y(y)?;
        let i = self.validate_i(y * self.x_len + x)?;
        Some(i)
    }
    fn get_at_i(&self, i: INT) -> Option<char> { Some(self.board[usize::try_from(self.validate_i(i)?).ok()?]) }
    fn get_at_xy(&self, x: INT, y: INT) -> Option<char> { self.get_at_i(self.xy_to_i(x, y)?) }
    fn get_word_at_xy(&self, x: INT, y: INT, len: INT, h_delta: INT, v_delta: INT) -> Option<String> {
        if h_delta == 0 && v_delta == 0 { return None; }
        let mut r = String::new();
        for i in 0..len {
            let x = x + h_delta * i;
            let y = y + v_delta * i;
            r.push(self.get_at_xy(x, y)?);
        }
        Some(r)
    }
    fn get_word_at_i(&self, i: INT, len: INT, h_delta: INT, v_delta: INT) -> Option<String> {
        let (x, y) = self.i_to_xy(i)?;
        self.get_word_at_xy(x, y, len, h_delta, v_delta)
    }
    fn get_all_words_with_len(&self, len: INT) -> impl Iterator<Item = String> + use<'_> {
        vec![(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)].into_iter().flat_map(move |(h_delta, v_delta)| {
            (0..self.get_board_len()).filter_map(move |i| {
                self.get_word_at_i(i, len, h_delta, v_delta)
            })
        })
    }
    fn get_count_of_word(&self, w: &str) -> INT { INT::try_from(self.get_all_words_with_len(INT::try_from(w.len()).unwrap()).filter(|v| w == v).count()).unwrap() }
    fn get_count_of_word_in_x_shape(&self, w: &str) -> INT {
        let len = INT::try_from(w.len()).unwrap();
        let f = len / 2;
        assert_eq!(1, len % 2);
        (0..self.get_board_len()).map(move |i| {
            let (x, y) = self.i_to_xy(i).unwrap();

            //1
            // let directions = vec![(-1,-1), (-1,1), (1,1), (1,-1)];
            // let direction_pairs = directions.clone().into_iter().zip(directions.into_iter().cycle().skip(1));
            // direction_pairs.map(move |((hd1, vd1),(hd2, vd2))| {
            //     let w1 = self.get_word_at_xy(x - hd1 * f, y - vd1 * f, len, hd1, vd1);
            //     let w2 = self.get_word_at_xy(x - hd2 * f, y - vd2 * f, len, hd2, vd2);
            //     match (w1, w2) {
            //         (Some(s1), Some(s2)) if s1 == w && s2 == w => 1,
            //         _ => 0
            //     }
            // })

            //2
            // vec![(-1,-1), (-1,1), (1,1), (1,-1), (-1,-1)].windows(2)
            //     .map(|a|(a
            //         .iter()
            //         .map(|(h_delta, v_delta)|
            //             self.get_word_at_xy(x - h_delta * f, y - v_delta * f, len, *h_delta, *v_delta))
            //         .all(|s| s == Some(w.to_string()))
            //     ) as INT).collect::<Vec<_>>()

            //3
            vec![(-1,-1), (-1,1), (1,1), (1,-1)].into_iter()
                .map(|(h_delta, v_delta)|
                    Some(w.to_string()) == self.get_word_at_xy(x - h_delta * f, y - v_delta * f, len, h_delta, v_delta)
                )
                .collect::<Vec<_>>().into_iter().cycle().take(5).collect::<Vec<_>>().windows(2)
                .map(|b| b.iter().all(|v| *v)).any(|b| b) as INT

        }).sum()
    }
}
fn main() {
    println!("{}", Crossword::new(get_data()).get_count_of_word(CROSSWORD_WORD1));
    println!("{}", Crossword::new(get_data()).get_count_of_word_in_x_shape(CROSSWORD_WORD2));
    println!("{}", Crossword::new(get_data()).get_count_of_word_in_x_shape("MAM")); // 504 (when multiple true,true pairs) vs 126 (when multiple true,true pairs are treated as 1)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn get_test_data_p1() -> &'static str {
"\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
    }
    fn get_test_p1_cw() -> Crossword { Crossword::new(&get_test_data_p1()) }
    #[test] fn test_p1() { assert_eq!(Crossword::new(get_data()).get_count_of_word(CROSSWORD_WORD1), 2718); }
    #[test] fn test_p2() { assert_eq!(Crossword::new(get_data()).get_count_of_word_in_x_shape(CROSSWORD_WORD2), 2046); }
    #[test] fn test_cw_x_len() { assert_eq!(get_test_p1_cw().x_len, 10); }
    #[test] fn test_cw_y_len() { assert_eq!(get_test_p1_cw().y_len, 10); }
    #[test] fn test_cw_get_word_01() { assert_eq!(get_test_p1_cw().get_word_at_xy(0, 0, 4, 1, 0).unwrap(), "MMMS"); }
    #[test] fn test_cw_get_word_02() { assert_eq!(get_test_p1_cw().get_word_at_xy(4, 0, 4, 1, 0).unwrap(), "XXMA"); }
    #[test] fn test_cw_get_word_03() { assert_eq!(get_test_p1_cw().get_word_at_xy(7, 0, 4, 1, 0), None); }
    #[test] fn test_cw_get_word_04() { assert_eq!(get_test_p1_cw().get_word_at_xy(2, 7, 4, 1, 0).unwrap(), "XAMA"); }
    #[test] fn test_cw_get_word_05() { assert_eq!(get_test_p1_cw().get_word_at_xy(5, 8, 4, 1, 0).unwrap(), "XMMM"); }
    #[test] fn test_cw_get_word_06() { assert_eq!(get_test_p1_cw().get_word_at_xy(6, 8, 4, 1, 0).unwrap(), "MMMM"); }
    #[test] fn test_cw_get_word_07() { assert_eq!(get_test_p1_cw().get_word_at_xy(0, 8, 10, 1, 0).unwrap(), "MAMMMXMMMM"); }
    #[test] fn test_cw_get_word_08() { assert_eq!(get_test_p1_cw().get_word_at_xy(0, 8, 11, 1, 0), None); }
    #[test] fn test_cw_get_word_09() { assert_eq!(get_test_p1_cw().get_word_at_xy(9, 0, 4, 0, 1).unwrap(), "MAMX"); }
    #[test] fn test_cw_get_word_10() { assert_eq!(get_test_p1_cw().get_word_at_xy(0, 0, 4, 1, 1).unwrap(), "MSXM"); }
    #[test] fn test_count_word() { assert_eq!(get_test_p1_cw().get_count_of_word(CROSSWORD_WORD1), 18); }
    #[test] fn test_cw_x_mas() { assert_eq!(get_test_p1_cw().get_count_of_word_in_x_shape(CROSSWORD_WORD2), 9); }
 }