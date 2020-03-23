use proconio::input;
use std::collections::BTreeMap;

fn nc2(n: i64) -> i64 {
    n * (n - 1) / 2
}

fn main() {
    input! {
        n: usize,
        av: [i64; n],
    };

    let mut bm = BTreeMap::new();
    for a in av.iter() {
        *bm.entry(a).or_insert(0) += 1_i64;
    }
    let sum = bm.iter().fold(0, |acc, (_, &v)| acc + nc2(v));
    for v in av.iter().map(|a| bm[a]) {
        let ans = sum - nc2(v) + nc2(v - 1);
        println!("{}", ans);
    }
}
