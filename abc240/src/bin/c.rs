use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    };
    let mut dp = BTreeSet::new();
    dp.insert(0);
    for (a, b) in ab {
        let mut next = BTreeSet::new();
        for x in dp {
            next.insert(x + a);
            next.insert(x + b);
        }
        dp = next;
    }
    let ans = dp.contains(&x);
    println!("{}", if ans { "Yes" } else { "No" });
}
