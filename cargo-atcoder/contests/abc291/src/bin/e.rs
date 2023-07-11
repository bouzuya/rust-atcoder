use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    };

    let mut set = HashSet::new();
    for (x, y) in xy {
        set.insert((x, y));
    }

    let mut edges = vec![vec![]; n];
    let mut count = vec![0; n];
    for (x, y) in set {
        edges[x].push(y);
        count[y] += 1;
    }

    // for e in edges.iter() {
    //     if e.len() > 1 {
    //         println!("No");
    //         return;
    //     }
    // }

    let mut deque = VecDeque::new();
    for u in 0..n {
        if count[u] == 0 {
            deque.push_back(u);
        }
    }
    if deque.len() != 1 {
        println!("No");
        return;
    }
    // println!("{:?}", edges);
    // println!("{:?}", count);
    // println!("{:?}", deque);

    let mut pos = vec![];
    while let Some(u) = deque.pop_front() {
        pos.push(u);
        for v in edges[u].iter().copied() {
            count[v] -= 1;
            if count[v] == 0 {
                deque.push_back(v);
            }
        }
        if deque.len() != 1 && pos.len() != n {
            println!("No");
            return;
        }
    }
    if pos.len() != n {
        println!("No");
        return;
    }

    // println!("{:?}", pos);

    println!("Yes");
    let mut ans = vec![0; n];
    for (i, p) in pos.iter().copied().enumerate() {
        ans[p] = i + 1;
    }
    for a in ans {
        println!("{}", a);
    }
}
