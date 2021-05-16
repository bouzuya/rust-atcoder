use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    };
    let mut max_c = 0;
    let mut c = 0;
    let mut p = h[0];
    for &h_i in h.iter().skip(1) {
        if p >= h_i {
            c += 1;
        } else {
            c = 0;
        }
        max_c = cmp::max(max_c, c);
        p = h_i;
    }
    let ans = max_c;
    println!("{}", ans);
}
