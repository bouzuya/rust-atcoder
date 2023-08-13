use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn bfs(edges: &[Vec<(usize, usize, usize)>], s: usize) -> Vec<Option<usize>> {
    let mut dist = vec![None; edges.len()];
    dist[s] = Some(0_usize);
    let mut deque = VecDeque::new();
    deque.push_back(s);
    while let Some(u) = deque.pop_front() {
        let dist_u = dist[u].unwrap();
        for (v, _, _) in edges[u].iter().copied().filter(|&(_, c, _)| c > 0) {
            if dist[v].is_none() {
                dist[v] = Some(dist_u + 1);
                deque.push_back(v);
            }
        }
    }
    dist
}

fn dfs(
    edges: &mut [Vec<(usize, usize, usize)>],
    u: usize,
    t: usize,
    f: usize,
    removed: &mut [usize],
    dist: &[Option<usize>],
) -> Option<usize> {
    if u == t {
        return Some(f);
    }
    while removed[u] < edges[u].len() {
        let (v, c, rev) = edges[u][removed[u]];
        if c > 0 && dist[u].unwrap() < dist[v].unwrap() {
            if let Some(flow) = dfs(edges, v, t, f.min(c), removed, dist) {
                edges[u][removed[u]].1 -= flow;
                edges[v][rev].1 += flow;
                return Some(flow);
            }
        }
        removed[u] += 1;
    }
    None
}

fn calc_max_flow(edges: &mut [Vec<(usize, usize, usize)>], s: usize, t: usize) -> usize {
    let mut flow = 0_usize;
    loop {
        let dist = bfs(&edges, s);
        if dist[t].is_none() {
            break;
        }
        let mut removed = vec![0; edges.len()];
        while let Some(f) = dfs(edges, s, t, std::usize::MAX, &mut removed, &dist) {
            flow += f;
        }
    }
    flow
}

fn main() {
    input! {
        v: usize,
        e: usize,
        uvc: [(Usize1, Usize1, usize); e],
    }

    // Dinic 法
    let mut edges = vec![vec![]; v];
    for (u, v, c) in uvc {
        let index_u = edges[u].len();
        let index_v = edges[v].len();
        edges[u].push((v, c, index_v));
        edges[v].push((u, 0, index_u));
    }

    let max_flow = calc_max_flow(&mut edges, 0, v - 1);
    println!("{}", max_flow);
}
