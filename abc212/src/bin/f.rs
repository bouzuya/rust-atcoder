use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abst: [(Usize1, Usize1, Usize1, Usize1); m],
        xyz: [(Usize1, Usize1, Usize1); q],
    }

    let mut e = vec![vec![]; n + m];
    for (i, (a_i, b_i, s_i, t_i)) in abst.iter().copied().enumerate() {
        e[a_i].push((n + i, s_i));
        e[n + i].push((b_i, t_i + 1));
    }
    for u in 0..n + m {
        e[u].sort_by_key(|(_, t)| t);
    }

    let mut t = BTreeMap::new();
    t.insert(0, 0);
    for (_, _, s_i, t_i) in abst.iter().copied() {
        t.insert(s_i, 0);
        t.insert(t_i + 1, 0);
    }
    for (x_i, _, z_i) in xyz.iter().copied() {
        t.insert(x_i, 0);
        t.insert(z_i, 0);
    }
    for (i, (_, v)) in t.iter_mut().enumerate() {
        *v = i;
    }

    let mut node = vec![];
    for i in 0..n {
        node.push((0, i, 0));
    }
    let mut e = vec![];

    let ans = n - a.len();
    println!("{}", ans);
}
