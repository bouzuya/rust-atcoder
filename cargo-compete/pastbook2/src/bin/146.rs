use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut edges = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        edges[u].push(v);
        edges[v].push(u);
    }
    edges
}

fn dfs(
    dp_w: &mut Vec<usize>,
    dp_b: &mut Vec<usize>,
    edges: &[Vec<usize>],
    modp: usize,
    u: usize,
    p: usize,
) {
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(dp_w, dp_b, edges, modp, v, u);

        dp_w[u] *= dp_w[v] + dp_b[v];
        dp_w[u] %= modp;
        dp_b[u] *= dp_w[v];
        dp_b[u] %= modp;
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let modp = 1_000_000_007_usize;
    let mut dp_w = vec![1_usize; n];
    let mut dp_b = vec![1_usize; n];
    let edges = adjacency_list(n, &ab);
    dfs(&mut dp_w, &mut dp_b, &edges, modp, 0, 0);
    let ans = (dp_w[0] + dp_b[0]) % modp;
    println!("{}", ans);
}
