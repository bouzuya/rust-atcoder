use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(ans: &mut usize, set: &mut Vec<bool>, edges: &[Vec<usize>], u: usize) {
    if *ans >= 1_000_000 {
        return;
    }
    set[u] = true;
    for v in edges[u].iter().copied() {
        if set[v] {
            continue;
        }
        *ans += 1;
        dfs(ans, set, edges, v);
    }
    set[u] = false;
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let edges = adjacency_list(n, &uv);

    let mut ans = 1_usize;
    let mut set = vec![false; n];
    set[0] = true;
    dfs(&mut ans, &mut set, &edges, 0);

    println!("{}", ans.min(1_000_000));
}
