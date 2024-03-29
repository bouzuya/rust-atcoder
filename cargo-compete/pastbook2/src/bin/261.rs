use proconio::{input, marker::Usize1};

fn bellman_ford_prime(
    capital_v: usize,
    edges: &[Vec<(usize, i64, i64, usize)>],
    s: usize,
) -> (Vec<i64>, Vec<usize>, Vec<usize>) {
    let inf = 1_i64 << 60;
    let mut dist = vec![inf; capital_v];
    dist[s] = 0;

    let mut prev_v = vec![0; capital_v];
    let mut prev_e = vec![0; capital_v];
    loop {
        let mut update = false;
        for u in 0..capital_v {
            if dist[u] == inf {
                continue;
            }
            for (i, (v, cap, cost, _)) in edges[u].iter().copied().enumerate() {
                if cap > 0 && dist[v] > dist[u] + cost {
                    dist[v] = dist[u] + cost;
                    update = true;

                    prev_v[v] = u;
                    prev_e[v] = i;
                }
            }
        }
        if !update {
            break;
        }
    }
    (dist, prev_v, prev_e)
}

fn main() {
    input! {
        capital_v: usize,
        capital_e: usize,
        mut capital_f: i64,
        vucd: [(Usize1, Usize1, i64, i64); capital_e],
    }

    let mut edges = vec![vec![]; capital_v];
    for (u, v, c, d) in vucd {
        let index_u = edges[u].len();
        let index_v = edges[v].len();
        edges[u].push((v, c, d, index_v));
        edges[v].push((u, 0, -d, index_u));
    }

    let mut min_cost = 0_i64;
    while capital_f > 0 {
        let (dist, prev_v, prev_e) = bellman_ford_prime(capital_v, &edges, 0);

        if dist[capital_v - 1] == 1_i64 << 60 {
            println!("-1");
            return;
        }

        let mut v = capital_v - 1;
        let mut f = capital_f;
        while v != 0 {
            let u = prev_v[v];
            let i = prev_e[v];
            let c = edges[u][i].1;
            f = f.min(c);
            v = u;
        }

        v = capital_v - 1;
        while v != 0 {
            let u = prev_v[v];
            let i = prev_e[v];
            let j = edges[u][i].3;
            edges[u][i].1 -= f;
            edges[v][j].1 += f;
            v = u;
        }

        min_cost += f * dist[capital_v - 1];
        capital_f -= f;
    }

    let ans = min_cost;
    println!("{}", ans);
}
