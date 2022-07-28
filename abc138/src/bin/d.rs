use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(edges: &[Vec<usize>], y: &[usize], dp: &mut Vec<usize>, u: usize, p: usize, w: usize) {
    dp[u] = w;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(edges, y, dp, v, u, w + y[v]);
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); n - 1],
        px: [(Usize1, usize); q],
    };

    let edges = adjacency_list(n, &ab);

    let mut y = vec![0; n];
    for (p_i, x_i) in px {
        y[p_i] += x_i;
    }

    let mut dp = vec![0; n];
    dfs(&edges, &y, &mut dp, 0, 0, y[0]);

    for a in dp {
        println!("{}", a);
    }
}
