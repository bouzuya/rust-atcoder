use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize; n],
    };

    let mut map = BTreeMap::new();
    for (w_i, s_i) in w
        .iter()
        .copied()
        .zip(s.iter().copied())
        .collect::<Vec<(usize, char)>>()
    {
        *map.entry(w_i).or_insert(0) += if s_i == '1' { -1 } else { 1 };
    }

    let count_ones = s.iter().copied().filter(|&s_i| s_i == '1').count() as i64;
    let mut count = count_ones;
    let mut max = count;
    for d in map.values() {
        count += d;
        max = max.max(count);
    }
    let ans = max.max(s.iter().copied().filter(|&s_i| s_i == '0').count() as i64);
    println!("{}", ans);
}
