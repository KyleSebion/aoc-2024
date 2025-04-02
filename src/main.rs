use std::fmt::Debug;
use std::iter::Sum;
use std::str::FromStr;
use num::PrimInt;

use std::cmp::{min, max};
use std::fs;
use std::path::PathBuf;

fn get_list_file_path() -> Result<PathBuf, String> {
    const LIST_TXT_FILE_NAME: &str = "list.txt";
    let mut cur_dir = fs::canonicalize(".").expect("canonicalize");
    loop {
        let ents = cur_dir.read_dir().expect("read_dir");
        for res_ent in ents {
            let ent = res_ent.expect("let ent").path();
            if ent.file_name().expect("file_name") == LIST_TXT_FILE_NAME {
                return Ok(ent);
            }
        }
        cur_dir = match cur_dir.parent() {
            Some(p) => p.to_path_buf(),
            None => return Err("not found".to_string())
        }
    }
}
fn fill_vecs_from_list_file<T>(vec_a: &mut Vec<T>, vec_b: &mut Vec<T>) where T: FromStr, T::Err: Debug {
    let list_path = get_list_file_path().expect("list_path");
    let list_path_content = fs::read_to_string(&list_path).expect("read failed");
    for line in list_path_content.lines() {
        if line.is_empty() { continue; }
        let mut iter = line.split_ascii_whitespace();
        vec_a.push(iter.next().expect("next a").parse().expect("parse a"));
        vec_b.push(iter.next().expect("next b").parse().expect("parse b"));
    }
}
fn get_sorted_copy<T>(slice: &[T]) -> Vec<T> where T: PrimInt {
    let mut vec = slice.to_vec();
    vec.sort();
    vec
}
fn part_1<T>(slice_a: &[T], slice_b: &[T]) -> T where T: PrimInt + Sum {
    let sorted_a = get_sorted_copy(slice_a);
    let sorted_b = get_sorted_copy(slice_b);
    (0..sorted_a.len()).map(|i|
        max(sorted_a[i], sorted_b[i]) - min(sorted_a[i], sorted_b[i])
    ).sum()
}
fn part_2<T>(slice_a: &[T], slice_b: &[T]) -> T where T: PrimInt + Sum {
    slice_a.iter().map(|n|
        *n * T::from(
            slice_b.iter().filter(|&v| *n == *v).count()
        ).expect("T::from")
    ).sum()
}
fn main() {
    type VecItemType = u32;
    let mut vec_a = Vec::<VecItemType>::new();
    let mut vec_b = Vec::<VecItemType>::new();
    fill_vecs_from_list_file(&mut vec_a, &mut vec_b);
    println!("{}", part_1(&vec_a, &vec_b)); //  1938424
    println!("{}", part_2(&vec_a, &vec_b)); // 22014209
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sorted_copy_u32() {
        let v1 = vec![ 580u32, 238, 958, 64, 661, 653, 530, 805, 905, 152, 801, 144, 165, 173, 882, 90, 941, 23, 512, 279 ];
        let v2 = get_sorted_copy(&v1);
        assert_ne!(v1, v2);
        std::mem::drop(v1);
        assert_eq!(v2, vec![ 23u32, 64, 90, 144, 152, 165, 173, 238, 279, 512, 530, 580, 653, 661, 801, 805, 882, 905, 941, 958 ]);
    }
    #[test]
    fn test_sorted_copy_i32() {
        let v1 = vec![ 90i32, 12, -989, -801, 829, -18, 565, 569, 395, -821, 532, 821, 398, -191, 21, 930, 489, -924, -614, -776 ];
        let v2 = get_sorted_copy(&v1);
        assert_ne!(v1, v2);
        std::mem::drop(v1);
        assert_eq!(v2, vec![ -989i32, -924, -821, -801, -776, -614, -191, -18, 12, 21, 90, 395, 398, 489, 532, 565, 569, 821, 829, 930 ]);
    }
    #[test]
    fn test_sorted_copy_u64() {
        let v1 = vec![ 18315382344312000000u64, 7500451590052440000, 16352811546347400000, 11419875304609100000, 6236239562592190000, 10022969317615300000, 2392271276618220000, 8850267040205930000, 561346964382961000, 1677820251494470000, 243534184862267000, 13220062296836300000, 13483857392921800000, 9904110942393890000, 3021981067963810000, 17398268494095800000, 12954776812739100000, 1142944798355530000, 13988880538423900000, 11553456664515800000 ];
        let v2 = get_sorted_copy(&v1);
        assert_ne!(v1, v2);
        std::mem::drop(v1);
        assert_eq!(v2, vec![ 243534184862267000u64, 561346964382961000, 1142944798355530000, 1677820251494470000, 2392271276618220000, 3021981067963810000, 6236239562592190000, 7500451590052440000, 8850267040205930000, 9904110942393890000, 10022969317615300000, 11419875304609100000, 11553456664515800000, 12954776812739100000, 13220062296836300000, 13483857392921800000, 13988880538423900000, 16352811546347400000, 17398268494095800000, 18315382344312000000 ]);
    }
    #[test]
    #[ignore = "just testing ignore"]
    fn test_sorted_copy_i64() {
        let v1 = vec![ -6100629294411516928i64, -8428527449691794432, -8446150723735934976, -2476416163306704896, -958135299923931136, -806905459475050496, -2961440001881905152, -6760730661650432000, -8797572845264281600, -6051582107275567104, 3940006005031176192, 2442548976471967232, 6735401930243641344, 7794894895243609088, 4636519503097085952, 7548662447934302208, 4856167822386820096, 5414117524601204736, 6720345766382870528, 2904545142897496576 ];
        let v2 = get_sorted_copy(&v1);
        assert_ne!(v1, v2);
        std::mem::drop(v1);
        assert_eq!(v2, vec![ -8797572845264281600i64, -8446150723735934976, -8428527449691794432, -6760730661650432000, -6100629294411516928, -6051582107275567104, -2961440001881905152, -2476416163306704896, -958135299923931136, -806905459475050496, 2442548976471967232, 2904545142897496576, 3940006005031176192, 4636519503097085952, 4856167822386820096, 5414117524601204736, 6720345766382870528, 6735401930243641344, 7548662447934302208, 7794894895243609088 ]);
    }
    #[test]
    fn test_part_1() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let b = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(part_1(&a, &b), 11);
    }
    #[test]
    fn test_part_2() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let b = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(part_2(&a, &b), 31);
    }
}