use std::cmp;

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dijkstra(n: usize, inf: usize, e: &Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let mut d = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for &v in e[u].iter() {
            let w = w_u + 1;
            if w < d[v] {
                d[v] = w;
                pq.push(std::cmp::Reverse((w, v)));
            }
        }
    }
    d
}

fn dfs(res: &mut usize, e: &Vec<Vec<usize>>, du: &Vec<usize>, dv: &Vec<usize>, u: usize, p: usize) {
    if du[u] < dv[u] {
        *res = cmp::max(*res, dv[u]);
    }
    for &v in e[u].iter() {
        if v == p {
            continue;
        }
        if du[v] >= dv[v] {
            continue;
        }
        dfs(res, e, du, dv, v, u);
    }
}

fn main() {
    input! {
        n: usize,
        u: Usize1,
        v: Usize1,
        ab: [(Usize1, Usize1); n - 1],
    };

    let e = adjacency_list(n, &ab);
    let inf = 1_000_000_000;
    let du = dijkstra(n, inf, &e, u);
    let dv = dijkstra(n, inf, &e, v);
    eprintln!("du = {:?}", du);
    eprintln!("dv = {:?}", dv);

    let mut res = 0;
    dfs(&mut res, &e, &du, &dv, u, u);

    let ans = if res > 0 { res - 1 } else { 0 };
    println!("{}", ans);
}
