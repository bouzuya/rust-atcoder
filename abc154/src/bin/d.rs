use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        k: usize,
        pv: [usize; n]
    };
    let mut sv = vec![0usize; n + 1];
    sv[0] = 0;
    for i in 0..pv.len() {
        sv[i + 1] = sv[i] + pv[i] + 1; // 整数で扱うため / 2 を最後にまとめる
    }
    let mut ms = 0;
    for i in 0..pv.len() {
        ms = max(ms, sv[i + 1] - sv[(i + 1).saturating_sub(k)]);
    }
    let ans = ms as f64 / 2f64;
    println!("{:.12}", ans);
}
