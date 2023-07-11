use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
        tx: [(usize, i64); q],
    };

    let mut set = BTreeSet::new();
    for (t, x) in tx {
        match t {
            1 => {
                set.insert(x);
            }
            2 => {
                let ans = match (set.range(1..x).rev().next(), set.range(x..).next()) {
                    (None, None) => -1,
                    (None, Some(y)) => (x - y).abs(),
                    (Some(y), None) => (x - y).abs(),
                    (Some(y), Some(z)) => (x - y).abs().min((x - z).abs()),
                };
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
