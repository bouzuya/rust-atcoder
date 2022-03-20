use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(size: &mut Vec<usize>, edges: &[Vec<usize>], u: usize, p: usize) {
    size[u] = 1;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(size, edges, v, u);
        size[u] += size[v];
    }
}

fn dfs2(sum: &mut usize, size: &[usize], edges: &[Vec<usize>], u: usize, p: usize) {
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs2(sum, size, edges, v, u);
    }
    *sum += size[u] * (size[0] - size[u]);
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };

    let edges = adjacency_list(n, &ab);

    let mut size = vec![0; n];
    dfs(&mut size, &edges, 0, 0);

    let mut ans = 0_usize;
    dfs2(&mut ans, &size, &edges, 0, 0);

    println!("{}", ans);
}
