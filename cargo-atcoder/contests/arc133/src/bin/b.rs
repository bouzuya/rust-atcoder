use std::cmp::Reverse;

use proconio::{input, marker::Usize1};
use superslice::Ext;

fn divisors(n: usize) -> Vec<usize> {
    let mut d = vec![];
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            d.push(i);
            if i != n / i {
                d.push(n / i);
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: [Usize1; n],
    };

    let mut pi = vec![n; n];
    for (i, p_i) in p.iter().copied().enumerate() {
        pi[p_i] = i;
    }

    let mut ij = vec![];
    for (j, q_j) in q.iter().copied().enumerate() {
        for k in divisors(q_j + 1) {
            ij.push((pi[k - 1], j));
        }
    }
    ij.sort_by_key(|&(i, j)| (i, Reverse(j)));
    let js = ij.into_iter().map(|(_, j)| j).collect::<Vec<usize>>();

    let mut dp = vec![js[0]];
    for j in js.iter().copied().skip(1) {
        if j > dp[dp.len() - 1] {
            dp.push(j);
        } else {
            let k = dp.lower_bound(&j);
            dp[k] = j;
        }
    }
    let ans = dp.len();
    println!("{}", ans);
}
