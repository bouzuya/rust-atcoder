use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut map = BTreeMap::new();
    for &a_i in a.iter() {
        *map.entry(a_i).or_insert(0) += 1;
    }
    let mut count = 0;
    for (_, v) in map {
        count += v * (v - 1) / 2;
    }

    let ans = n * (n - 1) / 2 - count;
    println!("{}", ans);
}
