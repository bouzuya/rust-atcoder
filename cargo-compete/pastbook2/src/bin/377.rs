use std::collections::VecDeque;

use im_rc::HashSet;
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

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        uvt: [(Usize1, Usize1, usize); m],
    }

    let inf = 1_usize << 60;
    let (edges, mut rev) = {
        let mut edges = vec![vec![]; n];
        for (u, v, t) in uvt {
            edges[u].push((v, t));
            edges[v].push((u, t));
        }

        let dist0 = dijkstra(n, inf, &edges, 0);
        let distn = dijkstra(n, inf, &edges, n - 1);
        let dist = dist0[n - 1];

        let mut edges2 = vec![vec![]; n];
        let mut rev = vec![HashSet::new(); n];
        for (u, edges_u) in edges.iter().enumerate() {
            for (v, t) in edges_u.iter().copied() {
                if dist0[u] + t + distn[v] == dist {
                    edges2[u].push(v);
                    rev[v].insert(u);
                }
            }
        }
        (edges2, rev)
    };

    let mut dp = vec![0_usize; n];
    let mut deque = VecDeque::new();
    dp[0] = a[0];
    deque.push_back(0);
    while let Some(u) = deque.pop_front() {
        for v in edges[u].iter().copied() {
            dp[v] = dp[v].max(dp[u] + a[v]);
            rev[v].remove(&u);
            if rev[v].is_empty() {
                deque.push_back(v);
            }
        }
    }

    let ans = dp[n - 1];
    println!("{}", ans);
}
