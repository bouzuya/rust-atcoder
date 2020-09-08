use proconio::input;
use proconio::marker::Usize1;

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

fn adjacency_list(n: usize, uvw: &Vec<(usize, usize, u64)>) -> Vec<Vec<(usize, u64)>> {
    let mut e = vec![vec![]; n];
    for &(u, v, w) in uvw.iter() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abt: [(Usize1, Usize1, u64); m],
    };

    let e = adjacency_list(n, &abt);

    let inf = 1_000_000_000_000_u64;
    let mut ans = inf;
    for i in 0..n {
        let d = dijkstra(n, inf, &e, i);
        ans = std::cmp::min(ans, d.into_iter().max().unwrap());
    }
    println!("{}", ans);
}
