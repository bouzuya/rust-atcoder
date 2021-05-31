use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        abcde: [(i64, i64, i64, i64, i64); n],
    };
    let mut max_score = 0;
    for (a_i, b_i, c_i, d_i, e_i) in abcde {
        max_score = cmp::max(max_score, (a_i + b_i + c_i + d_i) * 900 + e_i * 110);
    }
    let ans = max_score as f64 / 900_f64;
    println!("{}", ans);
}
