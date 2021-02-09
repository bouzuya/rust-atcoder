use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    };
    let mut map = BTreeMap::new();
    for &a_i in a.iter() {
        *map.entry(a_i).or_insert(0) += 1;
    }
    let mut counts = vec![];
    for (&_, &v) in map.iter() {
        counts.push(v);
    }
    counts.sort_by_key(|v| -v);
    let ans = counts.iter().skip(k).sum::<i64>();
    println!("{}", ans);
}
