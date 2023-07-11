use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    };

    let mut edges = vec![vec![]; n];
    for (a, b, c) in abc.iter().copied() {
        edges[a].push((b, c));
        edges[b].push((a, c));
    }

    let inf = 1_usize << 60;
    let start = 0;
    let mut dist = vec![inf; n];
    let mut prev = vec![HashSet::new(); n];
    let mut pq = BinaryHeap::new();
    dist[start] = 0;
    pq.push(Reverse((0, start)));
    while let Some(Reverse((w_u, u))) = pq.pop() {
        if w_u > dist[u] {
            continue;
        }
        for (v, w_v) in edges[u].iter().copied() {
            let w = w_u + w_v;
            if w == dist[v] {
                prev[v].insert(u);
            } else if w < dist[v] {
                dist[v] = w;
                prev[v] = {
                    let mut set = HashSet::new();
                    set.insert(u);
                    set
                };
                pq.push(Reverse((w, v)));
            }
        }
    }

    let mut ans = HashSet::new();
    let mut deque = VecDeque::new();
    deque.push_back(n - 1);
    ans.insert(n - 1);
    while let Some(u) = deque.pop_front() {
        for v in prev[u].iter().copied() {
            if ans.contains(&v) {
                continue;
            }
            deque.push_back(v);
            ans.insert(v);
        }
    }
    let ans = ans.len();
    println!("{}", ans);
}
