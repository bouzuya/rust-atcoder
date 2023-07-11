use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
        mut b: [i64; m],
    };
    a.sort();
    b.sort();

    let f = |a_i: i64, b_j: i64| -> i64 { (a_i - b_j).abs() };
    let mut min = f(a[0], b[0]);
    let mut i = 0;
    let mut j = 0;
    while i < n && j < m {
        let a_i = a[i];
        let b_j = b[j];
        min = cmp::min(min, f(a_i, b_j));
        if a_i < b_j {
            i += 1;
        } else {
            j += 1;
        }
    }

    let ans = min;
    println!("{}", ans);
}
