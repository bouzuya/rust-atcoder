use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
        a: [i64; n],
    };
    let mut max_a = vec![a[n - 1]; n];
    for (i, &a_i) in a.iter().enumerate().rev().skip(1) {
        max_a[i] = max(a_i, max_a[i + 1]);
    }
    let mut max_value = 0;
    let mut ans = 0;
    for i in 0..n - 1 {
        let v = max_a[i + 1] - a[i];
        if v > 0 {
            if v > max_value {
                max_value = v;
                ans = 1;
            } else if v == max_value {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
