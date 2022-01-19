use proconio::{input, marker::Usize1};

fn dfs(edges: &[Vec<(usize, usize)>], x: usize, u: usize, p: usize, d: usize) -> bool {
    if d == x {
        return true;
    }
    for (v, w) in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        if dfs(edges, x, v, u, d + w) {
            return true;
        }
    }
    false
}

fn adjacency_list(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut e = vec![vec![]; n];
    for (u, v, w) in uvw.iter().copied() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn main() {
    input! {
        n: usize,
        x: usize,
        abc: [(Usize1, Usize1, usize); n - 1],
    };

    let e = adjacency_list(n, &abc);
    for u in 0..n {
        if dfs(&e, x, u, u, 0) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
