use proconio::input;
use proconio::marker::Usize1;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        m: usize,
        lrs: [(Usize1, Usize1, i64); n],
    };
    let mut sum_s = 0;
    let mut sum = vec![0; m + 1];
    for &(l_i, r_i, s_i) in lrs.iter() {
        sum[l_i] += s_i;
        sum[r_i + 1] -= s_i;
        sum_s += s_i;
    }
    for i in 1..m {
        sum[i] += sum[i - 1];
    }
    sum.pop();

    let mut max_sum = 0;
    for i in 0..m {
        max_sum = max(max_sum, sum_s - sum[i]);
    }
    let ans = max_sum;
    println!("{}", ans);
}
