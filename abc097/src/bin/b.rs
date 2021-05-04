use std::cmp;

use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    let mut max_v = 1;
    for b in 1..=x {
        for p in 2..=x {
            match b.checked_pow(p as u32) {
                None => break,
                Some(v) => {
                    if v <= x {
                        max_v = cmp::max(max_v, v);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    let ans = max_v;
    println!("{}", ans);
}
