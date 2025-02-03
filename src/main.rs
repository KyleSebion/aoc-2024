#![allow(dead_code)]
use std::collections::HashSet;
use std::collections::HashMap;
fn ez1() -> &'static str {
    "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
"
}
fn ez2() -> &'static str {
    "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
"
}
fn ez3() -> &'static str {
    "\
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
"
}
fn ex() -> &'static str {
    "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"
}
fn data() -> &'static str {
    "\
654329854329876510123231016987212010510218565101234510343
789218765010945678754122987878900123424309654345679623490
543201698541232789669003456987210874535698701234388701581
650165467432871694578712012076321965549785698323897632672
569870346501960543209873453125409876678012587610432543543
678761250123451632114562164534012761237703476526501105412
879610167245893101023470079643443450349812345457432236903
965673298436734345652981188712565621056789400348901247894
434789567823321238761892199801874872341019511289187656785
323458478910210679850743056765923985432678521898096105016
012367320154301589012656145678910676563565430787145234127
121032110263469874322347238765876545612346541256230873298
678545021378954365411038389054985434501487632344321964343
569956985489565243506789872123010123676598701015423459652
654877676671278152105698143232131010989687772356910598701
123468964510189067834521032143328325678567887447827654323
003457853031656508921432541089419834509413996530988921013
012346322142347210560210654076508543210302105421243219034
329855412456898723874345789123467658723213489439850108123
410765001387129634987606932345358169654654398701763237654
567892187298034545674517801776549015456765217652304565456
103011296106234589503623410887232102367896900343215694347
234520345787105678212896556992185601298787871298344787298
309639876496221218923287647881096520109696678987650120156
018743212345430307832100238941087412368543567676543210347
329651008758745476542321107632101309454322430125210321298
478010129669876589401498210543291298765011321212345696212
566723234554745654320567641874980787012780876503856787101
345814656765634765210198532965673236323498903454945671012
210905549876589894321983449452360145324567612367834589323
356876632365565923015892158301450765410212143256521985434
547893421245674310234765067201221896784301032103410676965
432012560634789212105604300104334985895212349874306565876
101765676543434301789812215419455854326710458213215678945
569858987612343890176543216328766733210824567303764549432
478945890502345789265674307435432145601939665432876732981
323832781981256776328989438976965032789348778901905891070
210501632870125865410676521089874321011657345877814560565
343412541065434934541541078789289010150069256766923471234
454323401298743327601232369632103210541178129845812984321
565216543345652418796543454543432325652298087234701005630
074307232387651009187452330346501478743367896100565216787
189898101997843218012341021257562569895456545321074323896
234798001876960107345987012368876521080303434538989010143
145667010165456256296456543879989437871212343245698763230
021054323234387340187898534987654306965105650156787654321
430032104101298712345657659654789215456016781235673212452
545145235310132101076740348743690104312167098767654101963
696236996232145098889821289634587431203458129868910017876
587347887145096567986798678723876520104239234776521149887
678956981076787057872107509018987011015145445689432234796
454965438987789142963456410123070187676076323432011005645
567870127695654231254784321212165296589889916501123010434
458765476554514340345697654301254345432127807867834121125
389654389453001234434398941210345452345076321958985034016
210323430302112965525289030367676321456985430843476985107
323014321210227871012100123458983210167890121652105876898
"
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
struct Map {
    locs: Vec<Vec<char>>,
    trailheads: Vec<Point>,
}
impl Map {
    fn next_valid_step(c: &char) -> Option<char> {
        match c {
            ' ' => Some('0'),
            '0' => Some('1'),
            '1' => Some('2'),
            '2' => Some('3'),
            '3' => Some('4'),
            '4' => Some('5'),
            '5' => Some('6'),
            '6' => Some('7'),
            '7' => Some('8'),
            '8' => Some('9'),
            '9' => None,
            _ => None,
        }
    }
    const DIRECTIONS: [(usize, usize); 4] = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    fn new(d: &str) -> Self {
        let mut locs = Vec::new();
        let mut trailheads = Vec::new();
        for (y, l) in d.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in l.chars().enumerate() {
                row.push(c);
                if c == '0' {
                    trailheads.push(Point::new(x, y));
                }
            }
            locs.push(row);
        }
        Map { locs, trailheads }
    }
    fn get(&self, &Point { x, y }: &Point) -> Option<&char> {
        self.locs.get(y)?.get(x)
    }
    fn step(&self, point: &Point, pre: char) -> Option<Vec<Point>> {
        let cur = self.get(point)?;
        let ver = Self::next_valid_step(&pre)?;
        let nex = Self::next_valid_step(cur);
        if *cur != ver {
            None
        } else if nex.is_none() {
            Some(vec![*point])
        } else {
            let v = Self::DIRECTIONS
                .iter()
                .map(|(dx, dy)| Point::new(point.x.wrapping_add(*dx), point.y.wrapping_add(*dy)))
                .filter_map(|p| self.step(&p, *cur))
                .flatten()
                .collect::<Vec<_>>();
            if v.is_empty() {
                None
            } else {
                Some(v)
            }
        }
    }
    fn get_trailhead_9s_reached(&self, point: &Point) -> (Point, Vec<Point>, HashSet<Point>) {
        let v = self.step(point, ' ').unwrap_or_default();
        let h = HashSet::from_iter(v.iter().cloned());
        (*point, v, h)
    }
    fn get_trailheads_9s_reached(&self) -> HashMap<Point, (Point, Vec<Point>, HashSet<Point>)> {
        self.trailheads
            .iter()
            .map(|point| {
                (*point, self.get_trailhead_9s_reached(point))
            })
            .collect()
    }
    fn get_trailhead_score(&self, point: &Point) -> usize {
        self.get_trailhead_9s_reached(point).2.len()
    }
    fn get_trailheads_score(&self) -> usize {
        self.get_trailheads_9s_reached().iter().fold(0, |a, (_, set)| a + set.2.len())
    }
    fn get_trailhead_rating(&self, point: &Point) -> usize {
        self.get_trailhead_9s_reached(point).1.len()
    }
    fn get_trailheads_rating(&self) -> usize {
        self.get_trailheads_9s_reached().iter().fold(0, |a, (_, set)| a + set.1.len())
    }
}
fn main() {
    let m = Map::new(data());
    println!("{:?}", m.get_trailheads_rating());
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn ex_get() {
        let m = Map::new(ex());
        assert_eq!(m.get(&Point::new(0, 0)), Some(&'8'));
        assert_eq!(m.get(&Point::new(1, 0)), Some(&'9'));
        assert_eq!(m.get(&Point::new(2, 0)), Some(&'0'));
        assert_eq!(m.get(&Point::new(3, 0)), Some(&'1'));
        assert_eq!(m.get(&Point::new(4, 0)), Some(&'0'));
        assert_eq!(m.get(&Point::new(8, 0)), None);
        assert_eq!(m.get(&Point::new(7, 7)), Some(&'2'));
        assert_eq!(m.get(&Point::new(0, 8)), None);
    }
    #[test] fn p1_ez1() { assert_eq!(2, Map::new(ez1()).get_trailheads_score()); }
    #[test] fn p1_ez2() { assert_eq!(4, Map::new(ez2()).get_trailheads_score()); }
    #[test] fn p1_ez3() { assert_eq!(3, Map::new(ez3()).get_trailheads_score()); }
    #[test] fn p1_ex() { assert_eq!(36, Map::new(ex()).get_trailheads_score()); }
    #[test] fn p1_data() { assert_eq!(719, Map::new(data()).get_trailheads_score()); }
    #[test] fn p2_ex() { assert_eq!(81, Map::new(ex()).get_trailheads_rating()); }
    #[test] fn p2_data() { assert_eq!(1530, Map::new(data()).get_trailheads_rating()); }
}
