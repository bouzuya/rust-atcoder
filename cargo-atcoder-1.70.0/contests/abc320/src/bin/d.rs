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

    let mut ans = vec![None; n];
    ans[0] = Some((0_i64, 0_i64));
    let mut q = VecDeque::new();
    q.push_back(0_usize);
    while let Some(u) = q.pop_front() {
        let (x, y) = ans[u].unwrap();
        for (v, dx, dy) in edges[u].iter().copied() {
            let (nx, ny) = (x + dx, y + dy);
            match ans[v] {
                Some(_) => continue,
                None => {
                    ans[v] = Some((nx, ny));
                    q.push_back(v);
                }
            }
        }
    }

    for ans_i in ans.iter().copied() {
        match ans_i {
            None => println!("undecidable"),
            Some((x, y)) => println!("{} {}", x, y),
        }
    }
}
