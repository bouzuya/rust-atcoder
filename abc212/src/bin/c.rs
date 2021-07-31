use std::cmp;

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
        mut b: [i64; m],
    };
    a.sort();
    b.sort();

    let mut min = 1_000_000_000_000_000_i64;
    for a_i in a.into_iter() {
        let x = b.lower_bound(&a_i);
        let mut is = vec![];
        if (0..m).contains(&x) {
            is.push(x);
        }
        if x >= 1 {
            is.push(x - 1);
        }
        if x + 1 <= m - 1 {
            is.push(x + 1);
        }
        for i in is {
            min = cmp::min(min, (a_i - b[i]).abs());
        }
    }

    let ans = min;
    println!("{}", ans);
}
