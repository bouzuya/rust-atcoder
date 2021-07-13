use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut min = n;
    for b in 1.. {
        let b2 = 2_usize.pow(b as u32);
        if b2 > n {
            break;
        }
        let a = n / b2;
        let c = n - a * b2;
        min = cmp::min(min, a + b + c);
    }
    let ans = min;
    println!("{}", ans);
}
