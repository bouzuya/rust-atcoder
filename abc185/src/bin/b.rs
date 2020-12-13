use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n: i64,
        m: usize,
        t: i64,
        ab: [(i64, i64); m]
    };
    let max_n = n * 2;
    let mut c = n * 2;
    let mut p = 0;
    for (a_i, b_i) in ab {
        let x = (a_i - p) * 2;
        c -= x;
        if c <= 0 {
            println!("No");
            return;
        }
        c += (b_i - a_i) * 2;
        c = min(max_n, c);
        p = b_i;
    }
    let x = (t - p) * 2;
    c -= x;
    if c <= 0 {
        println!("No");
        return;
    }
    println!("Yes");
}
