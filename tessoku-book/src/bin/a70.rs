use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        xyz: [(Usize1, Usize1, Usize1); m],
    };

    let mut start = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        if a_i == 1 {
            start |= 1 << i;
        }
    }
    let goal = (1 << n) - 1;

    let mut edges = vec![vec![]; 1 << n];
    for (x, y, z) in xyz {
        let b = (1 << x) | (1 << y) | (1 << z);
        for u in 0..1 << n {
            let v = u ^ b;
            edges[u].push(v);
            edges[v].push(u);
        }
    }

    let inf = 1_usize << 60;
    let mut dist = vec![inf; 1 << n];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    dist[start] = 0_usize;
    while let Some(u) = queue.pop_front() {
        for v in edges[u].iter().copied() {
            if dist[v] == inf {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }

    let ans = if dist[goal] == inf {
        -1
    } else {
        dist[goal] as i64
    };
    println!("{}", ans);
}
