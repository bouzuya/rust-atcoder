use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        wv: [(i64, i64); n],
        x: [i64; m],
        lr: [(Usize1, Usize1); q],
    };
    for &(l_i, r_i) in lr.iter() {
        let mut y = vec![];
        for (j, &x_j) in x.iter().enumerate() {
            if l_i <= j && j <= r_i {
                continue;
            }
            y.push(x_j);
        }
        y.sort();
        let mut sum = 0;
        let mut set = BTreeSet::new();
        for &y_j in y.iter() {
            let mut max_k = 0;
            let mut max_v = 0;
            for (k, &(w_k, v_k)) in wv.iter().enumerate() {
                if w_k > y_j {
                    continue;
                }
                if set.contains(&k) {
                    continue;
                }
                if v_k > max_v {
                    max_k = k;
                    max_v = v_k;
                }
            }
            if max_v > 0 {
                set.insert(max_k);
            }
            sum += max_v;
        }
        println!("{}", sum);
    }
}
