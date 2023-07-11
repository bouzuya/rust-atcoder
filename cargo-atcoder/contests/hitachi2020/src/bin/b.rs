use proconio::input;
use proconio::marker::Usize1;
use std::cmp::min;

fn main() {
    input! {
        large_a: usize,
        large_b: usize,
        m: usize,
        a: [i64; large_a],
        b: [i64; large_b],
        xyc: [(Usize1, Usize1, i64); m],
    };
    let mut min_v = a.iter().min().unwrap() + b.iter().min().unwrap();
    for (x_i, y_i, c_i) in xyc {
        min_v = min(min_v, a[x_i] + b[y_i] - c_i);
    }
    let ans = min_v;
    println!("{}", ans);
}
