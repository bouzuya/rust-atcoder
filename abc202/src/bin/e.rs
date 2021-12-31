use proconio::{input, marker::Usize1};
use superslice::Ext;

fn dfs(
    d: usize,
    depth: &mut [Vec<usize>],
    p: usize,
    io: &mut [(usize, usize)],
    edge: &[Vec<usize>],
    u: usize,
) -> usize {
    io[u].0 = p;
    depth[d].push(p);
    let mut pos = p + 1;
    for v in edge[u].iter().copied() {
        pos = dfs(d + 1, depth, pos, io, edge, v);
    }
    io[u].1 = pos;
    pos + 1
}

fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1],
        q: usize,
        ud: [(Usize1, usize); q],
    };

    let mut edge = vec![vec![]; n];
    for (i, p_i) in p.into_iter().enumerate() {
        edge[p_i].push(i + 1);
    }

    let mut depth = vec![vec![]; n];
    let mut io = vec![(0, 0); n];
    dfs(0, &mut depth, 0, &mut io, &edge, 0);

    for (u, d) in ud {
        let (in_u, out_u) = io[u];
        let ans = depth[d].lower_bound(&out_u) - depth[d].lower_bound(&in_u);
        println!("{}", ans);
    }
}
