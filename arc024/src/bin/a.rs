use proconio::input;
use std::{cmp::min, collections::BTreeMap};

fn main() {
    input! {
        l_n: usize,
        r_n: usize,
        l: [i64; l_n],
        r: [i64; r_n],
    };
    let mut map_l = BTreeMap::new();
    for l_i in l {
        *map_l.entry(l_i).or_insert(0) += 1;
    }
    let mut map_r = BTreeMap::new();
    for r_i in r {
        *map_r.entry(r_i).or_insert(0) += 1;
    }
    let mut count = 0;
    for (k, count_l) in map_l {
        count += match map_r.get(&k) {
            None => 0,
            Some(&count_r) => min(count_l, count_r),
        }
    }
    let ans = count;
    println!("{}", ans);
}
