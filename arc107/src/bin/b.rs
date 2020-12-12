use proconio::input;
use std::cmp::min;

fn f(n: i64, k: i64) -> i64 {
    min(k - 1, 2 * n + 1 - k)
}

fn main() {
    input! {
        n: i64,
        k: i64,
    };
    let mut count = 0_i64;
    for x in 2_i64..=2 * n {
        let y = x - k;
        if y < 2 || 2 * n < y {
            continue;
        }
        count += f(n, x) * f(n, x - k);
    }
    let ans = count;
    println!("{}", ans);
}
