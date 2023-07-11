use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [i64; n],
        p: [i64; n],
    };
    let mut map = HashMap::new();
    for (c_i, p_i) in c.into_iter().zip(p.into_iter()) {
        let entry = map.entry(c_i).or_insert(p_i);
        *entry = (*entry).min(p_i);
    }
    if map.len() < k {
        println!("-1");
        return;
    }
    let mut cp = map.into_iter().collect::<Vec<(i64, i64)>>();
    cp.sort_by_key(|&(_, p)| p);
    let ans = cp.into_iter().take(k).map(|(_, p)| p).sum::<i64>();
    println!("{}", ans);
}
