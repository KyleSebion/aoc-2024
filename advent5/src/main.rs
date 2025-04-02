use std::{cmp::Ordering, collections::{HashMap, HashSet}, ops::{Deref, DerefMut}};
fn get_data() -> &'static str {
    include_str!("input.txt")
}
#[derive(Debug)]
struct PageLrs {
    page: u32,
    lefts: Vec<u32>,
    rights: Vec<u32>,
}
impl PageLrs {
    fn new(page: u32) -> PageLrs {
        PageLrs {
            page,
            lefts: Vec::new(),
            rights: Vec::new(),
        }
    }
}
#[derive(Debug)]
struct PageOrder {
    left: u32,
    right: u32,
}
impl PageOrder {
    fn new(left: u32, right: u32) -> Self {
        PageOrder { left, right }
    }
}
#[derive(Debug)]
struct PageOrders {
    vec: Vec<PageOrder>,
}
impl PageOrders {
    fn new() -> Self {
        PageOrders { vec: Vec::new() }
    }
    fn get_pages(&self) -> HashSet<u32> {
        self.iter().flat_map(|v|[v.left, v.right]).collect::<HashSet<_>>()
    }
    fn get_page_lrs(&self, page: u32) -> PageLrs {
        let mut plrs = PageLrs::new(page);
        for po in self.iter() {
            if page == po.left {
                plrs.rights.push(po.right);
            }
            if page == po.right {
                plrs.lefts.push(po.left);
            }
        }
        plrs
    }
    fn get_pages_lrs(&self) -> HashMap<u32, PageLrs> {
        self.get_pages().iter().map(|&p| (p, self.get_page_lrs(p))).collect::<HashMap<_, _>>()
    }
}
impl Deref for PageOrders {
    type Target = Vec<PageOrder>;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.vec
    }
}
impl DerefMut for PageOrders {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.vec
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PageSet {
    vec: Vec<u32>,
}
impl PageSet {
    fn new() -> Self {
        PageSet { vec: Vec::new() }
    }
}
impl Deref for PageSet {
    type Target = Vec<u32>;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.vec
    }
}
impl DerefMut for PageSet {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.vec
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PageSets {
    vec: Vec<PageSet>,
}
impl PageSets {
    fn new() -> Self {
        PageSets { vec: Vec::new() }
    }
}
impl Deref for PageSets {
    type Target = Vec<PageSet>;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.vec
    }
}
impl DerefMut for PageSets {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.vec
    }
}
fn process_data(data: &str) -> (PageOrders, PageSets) {
    let mut pos = PageOrders::new();
    let mut pss = PageSets::new();
    for l in data.lines() {
        if l.contains("|") {
            let v = l
                .split("|")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            pos.push(PageOrder::new(v[0], v[1]));
        } else if l.contains(",") {
            let mut ps = PageSet::new();
            for p in l.split(",").filter_map(|v| v.parse::<u32>().ok()) {
                ps.push(p);
            }
            pss.push(ps);
        }
    }
    (pos, pss)
}
fn is_correctly_ordered(pos: &PageOrders, ps: &PageSet) -> bool {
    for PageOrder { left: l, right: r } in pos.iter() {
        let li = ps.iter().position(|p| l == p);
        let ri = ps.iter().position(|p| r == p);
        if let (Some(li), Some(ri)) = (li, ri) {
            if li >= ri {
                return false;
            }
        }
    }
    true
}
fn get_correctly_and_incorrectly_ordered((pos, pss): (&PageOrders, &PageSets)) -> (PageSets, PageSets) {
    let mut cpss = PageSets::new();
    let mut ipss = PageSets::new();
    for u in pss.iter() {
        let u = u.clone();
        if is_correctly_ordered(pos, &u) {
            cpss.push(u);
        } else {
            ipss.push(u);
        }
    }
    (cpss, ipss)
}
fn get_sum_middle(pss: &PageSets) -> u32 {
    pss.iter().map(|ps| ps[ps.len() / 2]).sum()
}
fn fix_order(plrs: &HashMap<u32, PageLrs>, ps: &PageSet) -> PageSet {
    let mut cps = ps.clone();
    cps.sort_by(|a,b|{
        let a = &plrs[a];
        let b = &plrs[b];
        let alb = b.lefts.contains(&a.page);
        let arb = b.rights.contains(&a.page);
        let bla = a.lefts.contains(&b.page);
        let bra = a.rights.contains(&b.page);
        match (alb, arb, bla, bra) {
            ( true,  true,     _,     _) => panic!("a left and right of b"),
            (    _,     _,  true,  true) => panic!("b left and right of a"),
            ( true,     _,  true,     _) => panic!("a left of b and b left of a"),
            (    _,  true,     _,  true) => panic!("a right of b and b right of a"),
            ( true, false, false, false) => panic!("only a left b"),
            (false,  true, false, false) => panic!("only a right b"),
            (false, false,  true, false) => panic!("only b left a"),
            (false, false, false,  true) => panic!("only b right a"),
            (false, false, false, false) => panic!("no order?"),
            ( true, false, false,  true) => Ordering::Less,
            (false,  true,  true, false) => Ordering::Greater,
        }
    });
    cps
}
fn fix_orders(plrs: &HashMap<u32, PageLrs>, pss: &PageSets) -> PageSets {
    let mut cpss = PageSets::new();
    for ps in pss.iter() {
        cpss.push(fix_order(plrs, ps));
    }
    cpss
}
fn main() {
    let (pos, pss) = process_data(get_data());
    let (cpss, ipss) = get_correctly_and_incorrectly_ordered((&pos, &pss));
    let sum = get_sum_middle(&cpss);
    println!("{sum}");

    let plrs = pos.get_pages_lrs();
    let cpss = fix_orders(&plrs, &ipss);
    let sum = get_sum_middle(&cpss);
    println!("{sum}");
}
#[cfg(test)]
pub mod tests {
    use super::*;
    pub fn get_test_data_p1() -> &'static str {
        "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
    }
    #[test]
    fn test_p1_correctly_ordered_rows() {
        let (pos, pss) = process_data(get_test_data_p1());
        let (cpss, _) = get_correctly_and_incorrectly_ordered((&pos, &pss));
        let expected = PageSets {
            vec: vec![
                PageSet {
                    vec: vec![75, 47, 61, 53, 29],
                },
                PageSet {
                    vec: vec![97, 61, 53, 29, 13],
                },
                PageSet {
                    vec: vec![75, 29, 13],
                },
            ],
        };
        assert_eq!(cpss, expected);
    }
    #[test]
    fn test_p1_sum() {
        let (pos, pss) = process_data(get_test_data_p1());
        let (cpss, _) = get_correctly_and_incorrectly_ordered((&pos, &pss));
        let sum = get_sum_middle(&cpss);
        assert_eq!(sum, 143);
    }
    #[test]
    fn test_p1() {
        let (pos, pss) = process_data(get_data());
        let (cpss, _) = get_correctly_and_incorrectly_ordered((&pos, &pss));
        let sum = get_sum_middle(&cpss);
        assert_eq!(sum, 4872);
    }
    #[test]
    fn test_p2_test_data() {
        let (pos, pss) = process_data(get_test_data_p1());
        let (_, ipss) = get_correctly_and_incorrectly_ordered((&pos, &pss));
        let plrs = pos.get_pages_lrs();
        let cpss = fix_orders(&plrs, &ipss);
        let sum = get_sum_middle(&cpss);
        assert_eq!(sum, 123);
    }
    #[test]
    fn test_p2() {
        let (pos, pss) = process_data(get_data());
        let (_, ipss) = get_correctly_and_incorrectly_ordered((&pos, &pss));
        let plrs = pos.get_pages_lrs();
        let cpss = fix_orders(&plrs, &ipss);
        let sum = get_sum_middle(&cpss);
        assert_eq!(sum, 5564);
    }
}
