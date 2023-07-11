use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn bfs(n: usize, edges: &[Vec<usize>]) -> Vec<usize> {
    let mut color = 0_usize;
    let mut colors = vec![n; n];
    for start in 0..n {
        if colors[start] == n {
            let mut deque = VecDeque::new();
            deque.push_back(start);
            colors[start] = color;
            while let Some(u) = deque.pop_front() {
                for v in edges[u].iter().copied() {
                    if colors[v] == n {
                        colors[v] = color;
                        deque.push_back(v);
                    }
                }
            }
            color += 1;
        }
    }
    colors
}

fn main() {
    input! {
        n: usize,
        k: usize,
        l: usize,
        pq: [(Usize1, Usize1); k],
        rs: [(Usize1, Usize1); l],
    };

    let e1 = adjacency_list(n, &pq);
    let e2 = adjacency_list(n, &rs);

    let c1 = bfs(n, &e1);
    let c2 = bfs(n, &e2);

    let mut count = HashMap::new();
    for zipped in c1.iter().copied().zip(c2.iter().copied()) {
        *count.entry(zipped).or_insert(0) += 1;
    }

    for zipped in c1.iter().copied().zip(c2.iter().copied()) {
        println!("{}", count.get(&zipped).unwrap());
    }
}
