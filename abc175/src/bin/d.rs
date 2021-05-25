// <https://drken1215.hatenablog.com/entry/2020/08/17/182700>
use proconio::{input, marker::Usize1};
use std::iter;

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
        c: [i64; n],
    };
    let cycles = {
        let mut vv = vec![];
        let mut b = vec![false; n];
        for i in 0..n {
            if b[i] {
                continue;
            }
            let mut v = vec![];
            let mut j = i;
            while !b[j] {
                b[j] = true;
                v.push(c[j]);
                j = p[j];
            }
            vv.push(v);
        }
        vv
    };

    let mut ans = -1_000_000_000;
    for c in cycles {
        let s = iter::once(0)
            .chain(c.iter().chain(c.iter()).scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            }))
            .collect::<Vec<i64>>();
        let mut r = vec![-1_000_000_000; c.len()];
        for i in 0..c.len() {
            for j in 0..c.len() {
                chmax!(r[j], s[i + j] - s[i]);
            }
        }

        for l in 0..c.len() {
            if l > k {
                continue;
            }
            let q = (k - l) / c.len();
            if q == 0 && l == 0 {
                continue;
            }
            if s[c.len()] > 0 {
                chmax!(ans, s[c.len()] * q as i64 + r[l]);
            } else if l > 0 {
                chmax!(ans, r[l]);
            }
        }
    }
    println!("{}", ans);
}
