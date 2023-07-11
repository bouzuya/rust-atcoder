use proconio::input;
use std::cmp;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let mut max = 1;
    for x in 2..=b {
        let count = b / x - (a - 1) / x;
        if count >= 2 {
            max = cmp::max(max, x);
        }
    }

    let ans = max;
    println!("{}", ans);
}
