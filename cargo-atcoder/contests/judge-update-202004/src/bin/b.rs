use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xc: [(usize, char); n],
    };
    xc.sort_by_key(|&(u, c)| (Reverse(c), u));
    for (x_i, _) in xc {
        println!("{}", x_i);
    }
}
