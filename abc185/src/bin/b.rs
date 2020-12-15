use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: i64,
        m: usize,
        t: i64,
        ab: [(i64, i64); m]
    };
    let max_n = n;
    let mut c = n;
    let mut p = 0;
    for (a_i, b_i) in ab {
        c -= a_i - p;
        if c <= 0 {
            println!("No");
            return;
        }
        c = min(max_n, c + b_i - a_i);
        p = b_i;
    }
    c -= t - p;
    if c <= 0 {
        println!("No");
        return;
    }
    println!("Yes");
}
