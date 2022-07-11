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
    ans: &mut usize,
    used: &mut Vec<bool>,
    edges: &[Vec<usize>],
    len: usize,
    u: usize,
    p: usize,
) {
    if len == used.len() {
        *ans += 1;
        return;
    }

    for v in edges[u].iter().copied() {
        if v == p || used[v] {
            continue;
        }
        used[v] = true;
        dfs(ans, used, edges, len + 1, v, u);
        used[v] = false;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let edges = adjacency_list(n, &ab);
    let mut ans = 0;
    let mut used = vec![false; n];
    used[0] = true;
    dfs(&mut ans, &mut used, &edges, 1, 0, 0);
    println!("{}", ans);
}
