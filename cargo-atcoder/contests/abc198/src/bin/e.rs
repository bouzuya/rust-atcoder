use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(
    edges: &[Vec<usize>],
    c: &[usize],
    set: &mut BTreeSet<usize>,
    res: &mut Vec<bool>,
    u: usize,
    p: usize,
) {
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }

        let mut b = false;
        if set.insert(c[v]) {
            b = true;
            res[v] = true;
        }

        dfs(edges, c, set, res, v, u);

        if b {
            set.remove(&c[v]);
        }
    }
}

fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
        ab: [(Usize1, Usize1); n - 1],
    };

    let edges = adjacency_list(n, &ab);
    let mut set = BTreeSet::new();
    let mut res = vec![false; n];
    res[0] = true;
    set.insert(c[0]);
    dfs(&edges, &c, &mut set, &mut res, 0, 0);
    for a in res
        .iter()
        .copied()
        .enumerate()
        .filter(|&(_, b)| b)
        .map(|(i, _)| i + 1)
    {
        println!("{}", a);
    }
}
