use proconio::input;

fn dijkstra1(n: usize, a: usize, inf: usize, e: &[Vec<usize>], s: usize) -> Vec<usize> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut d = vec![inf; n];
    let mut pq = BinaryHeap::new();
    d[s] = 0;
    pq.push(Reverse((0, s)));
    while let Some(Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for (v, w_v) in e[u].iter().copied().enumerate() {
            let w = w_u + w_v * a;
            if w < d[v] {
                d[v] = w;
                pq.push(Reverse((w, v)));
            }
        }
    }
    d
}

fn dijkstra2(n: usize, b: usize, c: usize, inf: usize, e: &[Vec<usize>], s: usize) -> Vec<usize> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut d = vec![inf; n];
    let mut pq = BinaryHeap::new();
    d[s] = 0;
    pq.push(Reverse((0, s)));
    while let Some(Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for (v, w_v) in e[u].iter().copied().enumerate() {
            let w = w_u + w_v * b + c;
            if w < d[v] {
                d[v] = w;
                pq.push(Reverse((w, v)));
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        d: [[usize; n]; n],
    };

    let inf = 1_usize << 60;
    let dist1 = dijkstra1(n, a, inf, &d, 0);
    let dist2 = dijkstra2(n, b, c, inf, &d, n - 1);

    let mut min = inf;
    for v in 0..n {
        min = min.min(dist1[v] + dist2[v]);
    }

    let ans = min;
    println!("{}", ans);
}
