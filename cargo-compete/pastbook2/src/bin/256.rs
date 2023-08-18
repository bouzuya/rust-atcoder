use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn bfs(edges: &[Vec<(usize, usize, usize)>], start: usize) -> Vec<Option<usize>> {
    let mut dist = vec![None; edges.len()];
    dist[start] = Some(0_usize);
    let mut deque = VecDeque::new();
    deque.push_back(start);
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
    removed: &mut [usize],
    u: usize,
    t: usize,
    f: usize,
    dist: &[Option<usize>],
) -> Option<usize> {
    if u == t {
        return Some(f);
    }
    while removed[u] < edges[u].len() {
        let (v, c, rev) = edges[u][removed[u]];
        if c > 0 && dist[u].unwrap() < dist[v].unwrap() {
            if let Some(flow) = dfs(edges, removed, v, t, f.min(c), dist) {
                edges[u][removed[u]].1 -= flow;
                edges[v][rev].1 += flow;
                return Some(flow);
            }
        }
        removed[u] += 1;
    }
    None
}

fn calc_max_flow(edges: &mut [Vec<(usize, usize, usize)>], start: usize, goal: usize) -> usize {
    let mut flow = 0_usize;
    loop {
        let dist = bfs(edges, start);
        if dist[goal].is_none() {
            break;
        }

        let mut removed = vec![0; edges.len()];
        while let Some(f) = dfs(edges, &mut removed, start, goal, std::usize::MAX, &dist) {
            flow += f;
        }
    }
    flow
}

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(Usize1, Usize1, Usize1, Usize1); n],
    }

    let offset_row = 0;
    let offset_col = h;
    let offset_vertex_i = h + w;
    let offset_vertex_o = h + w + n;
    let offset_start = h + w + 2 * n;
    let offset_goal = h + w + 2 * n + 1;
    let mut edges = vec![vec![]; h + w + 2 * n + 2];
    for i in 0..h {
        let u = offset_start;
        let v = offset_row + i;
        let index_u = edges[u].len();
        let index_v = edges[v].len();
        edges[u].push((v, 1, index_v));
        edges[v].push((u, 0, index_u));
    }
    for i in 0..w {
        let u = offset_col + i;
        let v = offset_goal;
        let index_u = edges[u].len();
        let index_v = edges[v].len();
        edges[u].push((v, 1, index_v));
        edges[v].push((u, 0, index_u));
    }
    for (i, (a_i, b_i, c_i, d_i)) in abcd.iter().copied().enumerate() {
        let u = offset_vertex_i + i;
        let v = offset_vertex_o + i;
        let index_u = edges[u].len();
        let index_v = edges[v].len();
        edges[u].push((v, 1, index_v));
        edges[v].push((u, 0, index_u));

        for r in a_i..=c_i {
            let u = offset_row + r;
            let v = offset_vertex_i + i;
            let index_u = edges[u].len();
            let index_v = edges[v].len();
            edges[u].push((v, 1, index_v));
            edges[v].push((u, 0, index_u));
        }

        for c in b_i..=d_i {
            let u = offset_vertex_o + i;
            let v = offset_col + c;
            let index_u = edges[u].len();
            let index_v = edges[v].len();
            edges[u].push((v, 1, index_v));
            edges[v].push((u, 0, index_u));
        }
    }

    let max_flow = calc_max_flow(&mut edges, offset_start, offset_goal);
    println!("{}", max_flow);
}
