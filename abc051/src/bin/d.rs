use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, uvw: &Vec<(usize, usize, u64)>) -> Vec<Vec<(usize, u64)>> {
    let mut e = vec![vec![]; n];
    for &(u, v, w) in uvw.iter() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn dijkstra(n: usize, inf: u64, e: &Vec<Vec<(usize, u64)>>, s: usize) -> Vec<u64> {
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
        abc: [(Usize1, Usize1, u64); m],
    };

    let e = adjacency_list(n, &abc);
    let inf = 1_000_000_000;
    // d[u][v]: u から v への最短距離
    let mut d = vec![];
    for u in 0..n {
        d.push(dijkstra(n, inf, &e, u));
    }
    let mut ans = 0;
    for &(u, v, c) in abc.iter() {
        if d[u][v] != c {
            ans += 1;
        }
    }
    println!("{}", ans);
}
