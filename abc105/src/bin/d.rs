use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };

    let s = a
        .iter()
        .scan(0_usize, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<usize>>();
    let b = s.iter().map(|&s_i| s_i % m).collect::<Vec<usize>>();
    let mut map = BTreeMap::new();
    for &b_i in b.iter() {
        *map.entry(b_i).or_insert(0) += 1;
    }
    let mut count = 0_usize;
    for (k, v) in map {
        count += if k == 0 { v } else { 0 } + v * (v - 1) / 2;
    }
    let ans = count;
    println!("{}", ans);
}
