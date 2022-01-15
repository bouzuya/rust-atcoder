use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [i64; n],
        p: [i64; n],
    };
    let mut cp = c
        .into_iter()
        .zip(p.into_iter())
        .collect::<Vec<(i64, i64)>>();
    cp.sort_by_key(|&(_, p)| p);
    let mut ans = 0_i64;
    let mut used = HashSet::new();
    for (c, p) in cp {
        if used.len() >= k {
            break;
        }
        if used.insert(c) {
            ans += p;
        }
    }
    let ans = if used.len() >= k { ans } else { -1 };
    println!("{}", ans);
}
