use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };
    let mut c = vec![0; 1_000_001 + 1];
    for (a_i, b_i) in ab {
        c[a_i] += 1;
        c[b_i + 1] -= 1;
    }
    let mut max_x = 0;
    let mut x = 0;
    for &c_i in c.iter() {
        x += c_i;
        max_x = max(max_x, x);
    }

    let ans = max_x;
    println!("{}", ans);
}
