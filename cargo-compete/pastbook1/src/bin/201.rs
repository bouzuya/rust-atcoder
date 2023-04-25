use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Usize1};

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    }

    let mut edges = vec![HashSet::new(); n];
    let mut redges = vec![HashSet::new(); n];
    for (x, y) in xy {
        edges[x].insert(y);
        redges[y].insert(x);
    }

    let mut deque = VecDeque::new();
    for (i, e) in edges.iter().enumerate() {
        if e.is_empty() {
            deque.push_back(i);
        }
    }
    let mut dp = vec![0_usize; n];
    while let Some(v) = deque.pop_front() {
        for u in redges[v].iter().copied() {
            chmax!(dp[u], dp[v] + 1);
            edges[u].remove(&v);
            if edges[u].is_empty() {
                deque.push_back(u);
            }
        }
    }
    let ans = dp.iter().copied().max().unwrap();
    println!("{}", ans);
}
