use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut count = BTreeMap::new();
    for a_i in a {
        *count.entry(a_i - 1).or_insert(0) += 1;
        *count.entry(a_i).or_insert(0) += 1;
        *count.entry(a_i + 1).or_insert(0) += 1;
    }

    let ans = *count.values().max().unwrap();
    println!("{}", ans);
}
