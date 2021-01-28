use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        t: [i64; n],
    };
    let mut min_sum = 50 * 4;
    for bits in 0..1 << n {
        let mut sum = (0, 0);
        for i in 0..n {
            if (bits >> i) & 1 == 1 {
                sum.0 += t[i];
            } else {
                sum.1 += t[i];
            }
        }
        min_sum = min(min_sum, max(sum.0, sum.1));
    }
    let ans = min_sum;
    println!("{}", ans);
}
