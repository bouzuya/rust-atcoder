use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
        mut l: [i64; n],
    };
    l.sort();

    let mut ps = BTreeSet::new();
    for x_i in x.iter().copied() {
        for l_j in l.iter().copied() {
            ps.insert(x_i - l_j - 1);
            ps.insert(x_i + l_j);
        }
    }

    let mut ans = 0;
    let mut prev = 0;
    for p in ps {
        let mut d = x
            .iter()
            .copied()
            .map(|x_i| (x_i - p).abs())
            .collect::<Vec<i64>>();
        d.sort();
        if d.iter()
            .copied()
            .zip(l.iter().copied())
            .all(|(d_i, l_i)| d_i <= l_i)
        {
            ans += p - prev;
        }
        prev = p;
    }
    println!("{}", ans);
}
