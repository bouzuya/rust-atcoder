use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let b = a.iter().map(|&a_i| a_i % 200).collect::<Vec<i64>>();
    let mut map = BTreeMap::new();
    for &b_i in b.iter() {
        *map.entry(b_i).or_insert(0) += 1;
    }
    let mut count = 0_i64;
    for (_, v) in map {
        count += v * (v - 1);
    }
    let ans = count / 2;
    println!("{}", ans);
}
