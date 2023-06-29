use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut edges = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        edges[u].push(v);
        edges[v].push(u);
    }
    edges
}

fn dfs(ans: &mut Vec<usize>, edges: &[Vec<usize>], u: usize, p: usize) {
    ans[u] = 1_usize;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(ans, edges, v, u);
        ans[u] += ans[v];
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let edges = adjacency_list(n, &ab);
    let mut ans = vec![0_usize; n];
    dfs(&mut ans, &edges, 0, 0);

    for a in ans {
        println!("{}", a);
    }
}
