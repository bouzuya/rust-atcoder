use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }
    let mut p = h[0] - 1;
    for h_i in h.iter().copied().skip(1) {
        if p > h_i {
            println!("No");
            return;
        }

        p = cmp::max(p, h_i - 1);
    }
    println!("Yes");
}
