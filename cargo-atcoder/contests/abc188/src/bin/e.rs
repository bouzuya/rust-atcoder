use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
    }
    e
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        xy: [(Usize1, Usize1); m],
    };
    let edges = adjacency_list(n, &xy);
    let inf = 1_i64 << 60;
    let mut min = vec![inf; n];
    for u in 0..n {
        for v in edges[u].iter().copied() {
            min[v] = min[v].min(min[u].min(a[u]));
        }
    }
    let mut ans = -inf;
    for v in 0..n {
        ans = ans.max(a[v] - min[v]);
    }
    println!("{}", ans);
}
