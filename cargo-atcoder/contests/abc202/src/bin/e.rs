use proconio::{input, marker::Usize1};
use superslice::Ext;

fn dfs(
    depth: &mut Vec<Vec<usize>>,
    range: &mut Vec<(usize, usize)>,
    edges: &[Vec<usize>],
    u: usize,
    p: usize,
    d: usize,
    left: usize,
) -> usize {
    let mut right = left;
    depth[d].push(right);
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        right = dfs(depth, range, edges, v, u, d + 1, right + 1);
    }
    range[u] = (left, right);
    right
}

fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1],
        q: usize,
        ud: [(Usize1, usize); q],
    };

    let mut edges = vec![vec![]; n];
    for (i, p_i) in p.iter().copied().enumerate() {
        edges[p_i].push(i + 1);
        edges[i + 1].push(p_i);
    }

    let mut depth = vec![vec![]; n];
    let mut range = vec![(0, 0); n];
    dfs(&mut depth, &mut range, &edges, 0, 0, 0, 0);
    for (u, d) in ud {
        let (l, r) = range[u];
        let i_l = depth[d].lower_bound(&l);
        let i_r = depth[d].lower_bound(&(r + 1));
        let count = i_r - i_l;
        println!("{}", count);
    }
}
