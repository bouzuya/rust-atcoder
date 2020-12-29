use proconio::input;
use proconio::marker::Usize1;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };
    let mut q = vec![0; n];
    for (i, p_i) in p.into_iter().enumerate() {
        q[p_i] = i;
    }
    let mut max_l = 1;
    let mut l = 1;
    let mut p = q[0];
    for q_i in q.into_iter().skip(1) {
        if p > q_i {
            max_l = max(max_l, l);
            l = 0;
        }
        p = q_i;
        l += 1;
    }
    max_l = max(max_l, l);
    let ans = n - max_l;
    println!("{}", ans);
}
