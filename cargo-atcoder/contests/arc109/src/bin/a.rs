use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        a: i64,
        b: i64,
        x: i64,
        y :i64,
    };
    let ans = if a == b {
        x
    } else if a > b {
        min((a - b - 1) * y + x, (a - b - 1) * 2 * x + x)
    } else if a < b {
        min((b - a) * y + x, (b - a) * 2 * x + x)
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
