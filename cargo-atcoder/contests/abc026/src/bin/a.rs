use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        a: u32,
    };
    let mut max_v = 0;
    for x in 1..a {
        let y = a - x;
        max_v = max(max_v, x * y);
    }
    let ans = max_v;
    println!("{}", ans);
}
