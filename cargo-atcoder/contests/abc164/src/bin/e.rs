use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        uvab: [(Usize1, Usize1, usize, usize); m],
        cd: [(usize, usize); n]
    };

    let mut edges = vec![vec![]; n];
    for (u, v, a, b) in uvab.iter().copied() {
        edges[u].push((v, a, b));
        edges[v].push((u, a, b));
    }

    let max_cost = 50 * 50 + 50;
    let s = s.min(max_cost);
    let inf = 1_usize << 60;
    let mut min_time = vec![vec![inf; 3000 + 1]; n];
    min_time[0][s] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, 0, s)));
    while let Some(Reverse((time, vert, cost))) = pq.pop() {
        if min_time[vert][cost] != time {
            continue;
        }

        let (c, d) = cd[vert];
        if cost < max_cost {
            let next_vert = vert;
            let next_cost = (cost + c).min(max_cost);
            let next_time = time + d;
            if next_time < min_time[next_vert][next_cost] {
                min_time[next_vert][next_cost] = next_time;
                pq.push(Reverse((next_time, next_vert, next_cost)));
            }
        }

        for (next_vert, a, b) in edges[vert].iter().copied() {
            if cost < a {
                continue;
            }

            let next_cost = cost - a;
            let next_time = time + b;
            if next_time < min_time[next_vert][next_cost] {
                min_time[next_vert][next_cost] = next_time;
                pq.push(Reverse((next_time, next_vert, next_cost)));
            }
        }
    }

    for time in min_time.into_iter().skip(1) {
        let ans = time.into_iter().min().unwrap();
        println!("{}", ans);
    }
}
