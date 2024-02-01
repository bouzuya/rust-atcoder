use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        st: [(Usize1, Usize1); m]
    };

    let mut edges = vec![vec![]; n];
    for (i, (s, t)) in st.into_iter().enumerate() {
        edges[s].push((t, i));
    }

    let inf = 1_usize << 60;
    let mut dist = vec![(inf, n, n); n];
    dist[0] = (0_usize, 0_usize, m);
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), 0));
    while let Some((Reverse(d), u)) = pq.pop() {
        if d > dist[u].0 {
            continue;
        }
        for (v, i) in edges[u].iter().copied() {
            let next = (dist[u].0 + 1, u, i);
            if next.0 < dist[v].0 {
                dist[v] = next;
                pq.push((Reverse(next.0), v));
            }
        }
    }

    if dist[n - 1].0 == inf {
        for _ in 0..m {
            println!("-1");
        }
        return;
    }

    let mut set = HashSet::new();
    let mut cur = n - 1;
    while dist[cur].1 != cur {
        set.insert(dist[cur].2);
        cur = dist[cur].1;
    }

    for i in 0..m {
        if set.contains(&i) {
            let mut dist2 = vec![inf; n];
            dist2[0] = 0_usize;
            let mut pq = BinaryHeap::new();
            pq.push((Reverse(0), 0));
            while let Some((Reverse(d), u)) = pq.pop() {
                if d > dist2[u] {
                    continue;
                }
                for (v, j) in edges[u].iter().copied() {
                    if i == j {
                        continue;
                    }
                    let next = (d + 1, v);
                    if next.0 < dist2[v] {
                        dist2[v] = next.0;
                        pq.push((Reverse(next.0), next.1));
                    }
                }
            }

            if dist2[n - 1] == inf {
                println!("-1");
            } else {
                println!("{}", dist2[n - 1]);
            }
        } else {
            println!("{}", dist[n - 1].0);
        }
    }
}
