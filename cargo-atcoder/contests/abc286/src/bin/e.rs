use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn dijkstra(
    n: usize,
    inf: u64,
    a: &[usize],
    edges: &[Vec<usize>],
    start: usize,
) -> (Vec<u64>, Vec<usize>) {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut dist = vec![inf; n];
    let mut score = vec![0_usize; n];
    let mut pq = BinaryHeap::new();
    dist[start] = 0;
    score[start] = a[start];
    pq.push(Reverse((0, Reverse(a[start]), start)));
    while let Some(Reverse((w_u, Reverse(score_u), u))) = pq.pop() {
        if w_u > dist[u] {
            continue;
        }
        if score[u] > score_u {
            continue;
        }
        for v in edges[u].iter().copied() {
            let w = w_u + 1;
            if w < dist[v] {
                dist[v] = w;
                score[v] = score_u + a[v];
                pq.push(Reverse((w, Reverse(score_u + a[v]), v)));
            } else if w == dist[v] && score[v] < score_u + a[v] {
                score[v] = score_u + a[v];
                pq.push(Reverse((w, Reverse(score_u + a[v]), v)));
            }
        }
    }
    (dist, score)
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: [Chars; n],
        q: usize,
        uv: [(Usize1, Usize1); q],
    };

    let mut edges = vec![vec![]; n];
    for (i, s_i) in s.iter().enumerate() {
        for (j, s_ij) in s_i.iter().copied().enumerate() {
            if s_ij == 'Y' {
                edges[i].push(j);
            }
        }
    }

    let inf = 1 << 60;
    let mut dists = vec![];
    let mut scores = vec![];
    for i in 0..n {
        let (dist, score) = dijkstra(n, inf, &a, &edges, i);
        dists.push(dist);
        scores.push(score);
    }
    for (u, v) in uv {
        if dists[u][v] == inf {
            println!("Impossible");
        } else {
            println!("{} {}", dists[u][v], scores[u][v]);
        }
    }
}
