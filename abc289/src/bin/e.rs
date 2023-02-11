use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            c: [usize; n],
            uv: [(Usize1, Usize1); m],
        }

        let mut r_edges = vec![vec![]; n];
        let mut b_edges = vec![vec![]; n];
        for (u, v) in uv.iter().copied() {
            if c[v] == 0 {
                r_edges[u].push(v);
            } else {
                b_edges[u].push(v);
            }
            if c[u] == 0 {
                r_edges[v].push(u);
            } else {
                b_edges[v].push(u);
            }
        }

        let inf = 1_usize << 60;
        let mut dist = vec![inf; n * n];
        let get = |dist: &[usize], i: usize, j: usize| -> usize { dist[i * n + j] };
        let set = |dist: &mut Vec<usize>, i: usize, j: usize, v: usize| dist[i * n + j] = v;
        let mut deque = VecDeque::new();
        set(&mut dist, 0, n - 1, 0);
        deque.push_back((0, n - 1));
        while let Some((t, a)) = deque.pop_front() {
            let cur = get(&dist, t, a);
            for v1 in r_edges[t].iter().copied() {
                for v2 in b_edges[a].iter().copied() {
                    if get(&dist, v1, v2) == inf {
                        set(&mut dist, v1, v2, cur + 1);
                        deque.push_back((v1, v2));
                    }
                }
            }
            for v1 in b_edges[t].iter().copied() {
                for v2 in r_edges[a].iter().copied() {
                    if get(&dist, v1, v2) == inf {
                        set(&mut dist, v1, v2, cur + 1);
                        deque.push_back((v1, v2));
                    }
                }
            }
        }

        let ans = get(&dist, n - 1, 0);
        println!("{}", if ans == inf { -1 } else { ans as i64 });
    }
}
