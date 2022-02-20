// WA
use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(edges: &[Vec<usize>], ans: &mut Vec<(usize, usize)>, u: usize, p: usize, l: usize) -> usize {
    let mut r = l;
    let mut count = 0;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        r = dfs(edges, ans, v, u, l + count);
        count += 1;
    }
    ans[u] = (l, r);
    r
}

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    };

    let mut edges = adjacency_list(n, &uv);
    for i in 0..n {
        edges[i].sort();
    }

    let mut ans = vec![(0, 0); n];
    dfs(&edges, &mut ans, 0, 0, 0);
    let max = *ans.iter().map(|(a, b)| a.max(b)).max().unwrap();
    for (a, b) in ans {
        println!("{} {}", max - b + 1, max - a + 1);
    }
}
