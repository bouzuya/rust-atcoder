use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abxy: [(Usize1, Usize1, i64, i64); m],
    };

    let mut edges = vec![vec![]; n];
    for (a, b, x, y) in abxy {
        edges[a].push((b, x, y));
        edges[b].push((a, -x, -y));
    }

    let mut deque = VecDeque::new();
    let mut pos = vec![None; n];
    pos[0] = Some((0_i64, 0_i64));
    deque.push_back(0);
    while let Some(u) = deque.pop_front() {
        let (x, y) = pos[u].unwrap();
        for (v, dx, dy) in edges[u].iter().copied() {
            match pos[v] {
                Some(_) => continue,
                None => {
                    pos[v] = Some((x + dx, y + dy));
                    deque.push_back(v);
                }
            }
        }
    }

    for pos_i in pos {
        match pos_i {
            Some((x, y)) => println!("{} {}", x, y),
            None => println!("undecidable"),
        }
    }
}
