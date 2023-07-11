use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
        c: u64,
    };
    let mut min_v = 9999;
    for x in 0..=9999 {
        for y in 0..=(9999 - x) {
            if a * x + b * y > n {
                break;
            }
            let z = (n - (a * x + b * y)) / c;
            if a * x + b * y + c * z == n {
                let v = x + y + z;
                min_v = cmp::min(min_v, v);
            }
        }
    }
    let ans = min_v;
    println!("{}", ans);
}
