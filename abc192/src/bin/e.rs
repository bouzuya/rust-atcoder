use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, uvw: &Vec<(usize, usize, u64, u64)>) -> Vec<Vec<(usize, u64, u64)>> {
    let mut e = vec![vec![]; n];
    for &(u, v, w, t) in uvw.iter() {
        e[u].push((v, w, t));
        e[v].push((u, w, t));
    }
    e
}

fn dijkstra(n: usize, inf: u64, e: &Vec<Vec<(usize, u64, u64)>>, s: usize) -> Vec<u64> {
    let mut d = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for &(v, w_v, _) in e[u].iter() {
            let w = w_u + w_v;
            if w < d[v] {
                d[v] = w;
                pq.push(std::cmp::Reverse((w, v)));
            }
        }
    }
    d
}

fn dijkstra2(n: usize, inf: u64, e: &Vec<Vec<(usize, u64, u64)>>, s: usize) -> Vec<u64> {
    let mut d = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for &(v, w_v, t) in e[u].iter() {
            let w = w_u + w_v + if w_u % t == 0 { 0 } else { t - w_u % t };
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
        x: Usize1,
        y: Usize1,
        abtk: [(Usize1, Usize1, u64, u64); m],
    };

    let e = adjacency_list(n, &abtk);
    let inf = 1_000_000_000_000_000;
    let d = dijkstra(n, inf, &e, x);
    if d[y] == inf {
        println!("-1");
        return;
    }
    let d = dijkstra2(n, inf, &e, x);
    let ans = d[y];
    println!("{}", ans);
}
