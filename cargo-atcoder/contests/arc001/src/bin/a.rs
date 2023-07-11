use std::cmp;

use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        c: Bytes,
    };
    let mut max = 0;
    let mut min = n;
    for x in 1..=4 {
        let mut count = 0;
        for &c_i in c.iter() {
            if c_i == b'0' + x {
                count += 1;
            }
        }
        max = cmp::max(max, count);
        min = cmp::min(min, count);
    }
    println!("{} {}", max, min);
}
