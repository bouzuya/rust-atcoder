use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(used: &mut Vec<bool>, edges: &[Vec<usize>], u: usize, p: usize) -> bool {
    if edges[u].len() > 2 {
        return false;
    }

    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        if used[v] {
            return false;
        }
        used[v] = true;
        if !dfs(used, edges, v, u) {
            return false;
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let edges = adjacency_list(n, &uv);
    let mut used = vec![false; n];
    used[0] = true;
    let ans = dfs(&mut used, &edges, 0, 0) && used.into_iter().all(|b| b);
    println!("{}", if ans { "Yes" } else { "No" });
}
