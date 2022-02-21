use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn leaf_dfs(edges: &[Vec<usize>], leaf: &mut Vec<usize>, u: usize, p: usize) {
    let mut count = 0;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        leaf_dfs(edges, leaf, v, u);
        count += 1;
    }
    if count == 0 {
        leaf.push(u);
    }
}

fn minmax_dfs(edges: &[Vec<usize>], ans: &mut Vec<(usize, usize)>, u: usize, p: usize) {
    let (mut min, mut max) = ans[u];
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        minmax_dfs(edges, ans, v, u);
        min = min.min(ans[v].0);
        max = max.max(ans[v].1);
    }
    ans[u] = (min, max);
}

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    };

    let edges = adjacency_list(n, &uv);

    let mut leaf = vec![];
    leaf_dfs(&edges, &mut leaf, 0, 0);

    let mut ans = vec![(n, 0); n];
    for (i, leaf_i) in leaf.iter().copied().enumerate() {
        ans[leaf_i] = (i + 1, i + 1);
    }
    minmax_dfs(&edges, &mut ans, 0, 0);
    for (a, b) in ans {
        println!("{} {}", a, b);
    }
}
