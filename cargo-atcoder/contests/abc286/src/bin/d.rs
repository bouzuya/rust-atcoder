use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    };
    let mut dp = HashSet::new();
    dp.insert(0);
    for (a, b) in ab {
        let mut next = HashSet::new();
        for v in dp {
            for i in 0..=b {
                if v + i * a > x {
                    break;
                }
                next.insert(v + i * a);
            }
        }
        dp = next;
    }
    let ans = dp.contains(&x);
    println!("{}", if ans { "Yes" } else { "No" });
}
