use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };

    // same
    let mut ans1 = 1_000_000_000;
    for &(a_i, b_i) in ab.iter() {
        ans1 = min(ans1, a_i + b_i);
    }

    // not same
    let ans2 = {
        let mut min_a = ab[0].0;
        let mut min_i = 0;
        for (i, &(a_i, _)) in ab.iter().enumerate() {
            if a_i < min_a {
                min_i = i;
                min_a = a_i;
            }
        }
        let mut min_b = 1_000_000_000;
        for (i, &(_, b_i)) in ab.iter().enumerate() {
            if i == min_i {
                continue;
            }
            if b_i < min_b {
                min_b = b_i;
            }
        }
        max(min_a, min_b)
    };
    let ans3 = {
        let mut min_b = ab[0].1;
        let mut min_i = 0;
        for (i, &(_, b_i)) in ab.iter().enumerate() {
            if b_i < min_b {
                min_i = i;
                min_b = b_i;
            }
        }
        let mut min_a = 1_000_000_000;
        for (i, &(a_i, _)) in ab.iter().enumerate() {
            if i == min_i {
                continue;
            }
            if a_i < min_a {
                min_a = a_i;
            }
        }
        max(min_a, min_b)
    };

    let &ans = vec![ans1, ans2, ans3].iter().min().unwrap();
    println!("{}", ans);
}
