use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uvw: &[(usize, usize, u64)]) -> Vec<Vec<(usize, u64)>> {
    let mut e = vec![vec![]; n];
    for (u, v, w) in uvw.iter().copied() {
        e[u].push((v, w));
    }
    e
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, u64); m],
    };
    let edges = adjacency_list(n, &abc);
    let inf = 1_000_000_000_000;
    let mut dist = vec![vec![inf; n]; n];
    for k in 0..n {
        dist[k][k] = 0;
        for (v, w) in edges[k].iter().copied() {
            dist[k][v] = dist[k][v].min(w);
        }
    }

    let mut sum = 0_u64;
    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                dist[u][v] = dist[u][v].min(dist[u][k] + dist[k][v]);
                dist[v][u] = dist[v][u].min(dist[v][k] + dist[k][u]);
            }
        }

        for u in 0..n {
            for v in 0..n {
                sum += if dist[u][v] == inf { 0 } else { dist[u][v] };
            }
        }
    }

    let ans = sum;
    println!("{}", ans);
}
