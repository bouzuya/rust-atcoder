use proconio::{input, marker::Usize1};

fn dijkstra(n: usize, inf: usize, e: &[Vec<(usize, usize)>], s: usize) -> Vec<usize> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut d = vec![inf; n];
    let mut pq = BinaryHeap::new();
    d[s] = 0;
    pq.push(Reverse((0, s)));
    while let Some(Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for (v, w_v) in e[u].iter().copied() {
            let w = w_u + w_v;
            if w < d[v] {
                d[v] = w;
                pq.push(Reverse((w, v)));
            }
        }
    }
    d
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
        let d = dijkstra(n, 1_000_000_000, &e, u);
        if d.contains(&x) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
