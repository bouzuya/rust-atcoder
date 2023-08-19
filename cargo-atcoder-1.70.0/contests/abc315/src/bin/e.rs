use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn topological_sort_prime(e: &Vec<Vec<usize>>, visited: &[bool]) -> Option<Vec<usize>> {
    let n = e.len();
    let mut c_in = vec![0; n];
    for e_u in e.iter() {
        for &v in e_u.iter() {
            c_in[v] += 1;
        }
    }
    let mut q = VecDeque::new();
    for u in 0..n {
        if c_in[u] == 0 && visited[u] {
            q.push_back(u);
        }
    }
    let mut res = vec![];
    while let Some(u) = q.pop_front() {
        res.push(u);
        for &v in e[u].iter() {
            if !visited[v] {
                continue;
            }
            c_in[v] -= 1;
            if c_in[v] == 0 {
                q.push_back(v);
            }
        }
    }
    Some(res)
}

fn main() {
    input! {
        n: usize,
    };

    let mut p = vec![vec![]; n];
    for i in 0..n {
        input! {
            c_i: usize,
            p_i: [Usize1; c_i],
        }
        p[i] = p_i;
    }

    let mut rev = vec![vec![]; n];
    let mut edges = vec![vec![]; n];
    for v in 0..n {
        for u in p[v].iter().copied() {
            if u != 0 {
                edges[u].push(v);
            }
            rev[v].push(u);
        }
    }

    let mut deque = VecDeque::new();
    let mut visited = vec![false; n];
    visited[0] = true;
    deque.push_back(0);
    while let Some(u) = deque.pop_front() {
        for v in rev[u].iter().copied() {
            if visited[v] {
                continue;
            }
            visited[v] = true;
            deque.push_back(v);
        }
    }

    let sorted = topological_sort_prime(&edges, &visited).unwrap();
    for a in sorted.iter().take(sorted.len() - 1) {
        println!("{}", a + 1);
    }
}
