use std::cmp::min;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        h: [u64; n],
        c: [Usize1; k],
        ab: [(Usize1, Usize1); m],
    };

    let mut e_in = vec![vec![]; n];
    for &(a_i, b_i) in ab.iter() {
        if h[a_i] > h[b_i] {
            e_in[b_i].push(a_i);
        } else if h[a_i] < h[b_i] {
            e_in[a_i].push(b_i);
        } else {
            unreachable!("invalid input");
        }
    }

    let inf = m + 1;
    let mut d = vec![inf; n];
    for &c_i in c.iter() {
        d[c_i] = 0;
    }

    let mut hi = h
        .iter()
        .enumerate()
        .map(|(i, &h_i)| (h_i, i))
        .collect::<Vec<(u64, usize)>>();
    hi.sort();
    for &(_, v) in hi.iter() {
        for &u in e_in[v].iter() {
            d[u] = min(d[u], d[v] + 1);
        }
    }

    for &d_i in d.iter() {
        println!("{}", if d_i == inf { -1 } else { d_i as i64 });
    }
}
