use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };

    let mut ans = 1_000_000_000;
    for a in 0..n {
        for b in 0..n {
            ans = min(
                ans,
                if a == b {
                    ab[a].0 + ab[b].1
                } else {
                    max(ab[a].0, ab[b].1)
                },
            );
        }
    }
    println!("{}", ans);
}
