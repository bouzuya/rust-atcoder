use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
    };
    // (a + b) - (c + d) = k
    let k = if k < 0 { -k } else { k } as usize;
    let mut x = vec![0; 2 * n + 1];
    for v in 2..=2 * n {
        x[v] = cmp::min(v - 1, 2 * n + 1 - v);
    }
    let mut ans = 0_usize;
    for v in 0..=2 * n - k {
        ans += x[v] * x[v + k];
    }
    println!("{}", ans);
}
