use std::cmp::Reverse;

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    };
    xy.sort_by_key(|&(x_i, y_i)| (x_i, Reverse(y_i)));
    let y = xy.into_iter().map(|(_, y_i)| y_i).collect::<Vec<usize>>();
    let mut dp = vec![];
    for y_i in y {
        let j = dp.lower_bound(&y_i);
        if j < dp.len() {
            dp[j] = y_i;
        } else {
            dp.push(y_i);
        }
    }
    let ans = dp.len();
    println!("{}", ans);
}
