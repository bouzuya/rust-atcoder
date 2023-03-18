use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        k: usize,
    };

    let mut edges = vec![vec![]; k];
    for i in 0..k {
        edges[i].push(((i + 1) % k, 1));
        edges[i].push(((i * 10) % k, 0));
    }

    let inf = 1_usize << 60;
    let mut dist = vec![inf; k];
    let mut deque = VecDeque::new();
    deque.push_back((1, 0));
    dist[1] = 0;
    while let Some((u, d_u)) = deque.pop_front() {
        if u == 0 {
            break;
        }
        for (v, w_v) in edges[u].iter().copied() {
            let d_v = d_u + w_v;
            if d_v >= dist[v] {
                continue;
            }
            dist[v] = d_v;
            match w_v {
                0 => deque.push_front((v, d_v)),
                1 => deque.push_back((v, d_v)),
                _ => unreachable!(),
            }
        }
    }

    let ans = dist[0] + 1;
    println!("{}", ans);
}
