use proconio::{input, marker::Usize1};

fn bellman_ford(n: usize, inf: i64, edges: &[Vec<(usize, i64)>], start: usize) -> Option<Vec<i64>> {
    let mut dist = vec![inf; n];
    dist[start] = 0_i64;
    for i in 0..n {
        for (u, e_u) in edges.iter().enumerate() {
            for (v, w) in e_u.iter().copied() {
                if dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                    if i == n - 1 && v == n - 1 {
                        return None;
                    }
                }
            }
        }
    }
    Some(dist)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m],
    };

    let mut edges = vec![vec![]; n];
    for (a, b, c) in abc {
        edges[a].push((b, -c));
    }

    match bellman_ford(n, 1_i64 << 60, &edges, 0) {
        None => println!("inf"),
        Some(dist) => println!("{}", -dist[n - 1]),
    }
}
