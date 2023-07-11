use std::cmp;

use proconio::input;

fn main() {
    input! {
        k: usize,
    };
    if k % 9 != 0 {
        println!("0");
        return;
    }

    let p = 1_000_000_007;
    let mut sum = vec![0; k + 1];
    sum[0] = 1;
    for i in 1..=k {
        for j in 1..=cmp::min(i, 9) {
            sum[i] += sum[i - j];
            sum[i] %= p;
        }
    }

    let ans = sum[k];
    println!("{}", ans);
}
