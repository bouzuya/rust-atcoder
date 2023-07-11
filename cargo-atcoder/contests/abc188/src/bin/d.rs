use std::collections::{BTreeMap, BTreeSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        capital_c: i64,
        mut abc: [(Usize1, Usize1, i64); n],
    };
    abc.sort();
    let map = {
        let mut set = BTreeSet::new();
        for (a_i, b_i, _) in abc.iter().copied() {
            set.insert(a_i);
            set.insert(b_i + 1);
        }
        set.into_iter()
            .enumerate()
            .fold(BTreeMap::new(), |mut m, (i, k)| {
                m.insert(k, i);
                m
            })
    };
    let mut cum = vec![(0, 0_i64); map.len() + 1];
    for (a_i, b_i, c_i) in abc.iter().copied() {
        cum[map[&a_i]].0 = a_i;
        cum[map[&a_i]].1 += c_i;
        cum[map[&(b_i + 1)]].0 = b_i + 1;
        cum[map[&(b_i + 1)]].1 -= c_i;
    }
    for i in 0..cum.len() - 1 {
        cum[i + 1].1 += cum[i].1;
    }

    let mut ans = 0_i64;
    for (i, cum_i) in cum.iter().copied().take(cum.len() - 2).enumerate() {
        let d = cum[i + 1].0 - cum[i].0;
        let v = d as i64 * cum_i.1.min(capital_c);
        ans += v;
    }

    println!("{}", ans);
}
