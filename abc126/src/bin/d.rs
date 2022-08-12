use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut e = vec![vec![]; n];
    for (u, v, w) in uvw.iter().copied() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn dfs(color: &mut Vec<usize>, edges: &[Vec<(usize, usize)>], u: usize, p: usize) {
    for (v, w) in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        color[v] = (color[u] + w) % 2;
        dfs(color, edges, v, u);
    }
}

fn main() {
    input! {
        n: usize,
        uvw: [(Usize1, Usize1, usize); n - 1],
    };

    let edges = adjacency_list(n, &uvw);
    let mut color = vec![2; n];
    color[0] = 0;
    dfs(&mut color, &edges, 0, 0);
    for c in color {
        println!("{}", c);
    }
}
