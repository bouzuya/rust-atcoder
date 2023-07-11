use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [i64; n - 1],
        x: [i64; m],
    };

    let mut b = vec![0];
    for (i, s_i) in s.iter().copied().enumerate() {
        b.push(s_i - b[i]);
    }

    let mut map = HashMap::new();
    for (i, b_i) in b.iter().copied().enumerate() {
        let sgn = if i % 2 == 0 { 1 } else { -1 };
        for x_j in x.iter().copied() {
            *map.entry(sgn * (x_j - b_i)).or_insert(0) += 1_usize;
        }
    }

    let ans = *map.values().max().unwrap();
    println!("{}", ans);
}
