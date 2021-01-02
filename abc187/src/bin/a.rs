use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let mut sum_a = 0;
    for a_i in a.to_string().chars() {
        sum_a += (a_i as u8 - '0' as u8) as usize
    }
    let mut sum_b = 0;
    for b_i in b.to_string().chars() {
        sum_b += (b_i as u8 - '0' as u8) as usize
    }
    let ans = max(sum_a, sum_b);
    println!("{}", ans);
}
