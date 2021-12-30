use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut e = vec![vec![]; n];
    for (u, v, w) in uvw.iter().copied() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn dijkstra(n: usize, inf: usize, e: &[Vec<(usize, usize)>], s: usize) -> Vec<usize> {
    let mut d = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for &(v, w_v) in e[u].iter() {
            let w = w_u + w_v;
            if w < d[v] {
                d[v] = w;
                pq.push(std::cmp::Reverse((w, v)));
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Usize1,
        t: Usize1,
        uvab: [(Usize1, Usize1, usize, usize); m],
    };

    let edge_a = adjacency_list(
        n,
        &uvab
            .iter()
            .copied()
            .map(|(u, v, a, _)| (u, v, a))
            .collect::<Vec<(usize, usize, usize)>>(),
    );
    let edge_b = adjacency_list(
        n,
        &uvab
            .iter()
            .copied()
            .map(|(u, v, _, b)| (u, v, b))
            .collect::<Vec<(usize, usize, usize)>>(),
    );
    let inf = 1_000_000_000_000_000;
    let cost_a = dijkstra(n, inf, &edge_a, s);
    let cost_b = dijkstra(n, inf, &edge_b, t);
    let mut ans = vec![inf; n + 1];
    for i in (0..n).rev() {
        ans[i] = ans[i + 1].min(cost_a[i] + cost_b[i]);
    }

    for a in ans.into_iter().take(n) {
        println!("{}", 1_000_000_000_000_000 - a);
    }
}
