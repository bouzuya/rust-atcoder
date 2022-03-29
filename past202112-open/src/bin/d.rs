use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    };
    let mut ab = a
        .into_iter()
        .zip(b.into_iter())
        .enumerate()
        .collect::<Vec<_>>();
    ab.sort_by_key(|&(i, (a_i, b_i))| (Reverse(a_i + b_i), Reverse(a_i), i));
    for (i, _) in ab {
        println!("{}", i + 1);
    }
}
