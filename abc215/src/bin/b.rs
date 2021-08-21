use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut max = 0_usize;
    for k in 0.. {
        match 2_usize.checked_pow(k as u32) {
            Some(x) => {
                if x > n {
                    break;
                }
            }
            None => break,
        }
        max = cmp::max(max, k);
    }
    let ans = max;
    println!("{}", ans);
}
