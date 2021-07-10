// WA
use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
    }
    e
}

fn dfs(e: &[Vec<usize>], r: &mut Vec<usize>, used: &mut Vec<usize>, u: usize) {
    if e[u].is_empty() {
        r[u] = 2; // lose
        return;
    }
    if used[u] > 2 {
        return;
    }
    if used[u] == 1 {
        r[u] = 3; // draw
    }
    used[u] += 1;

    let mut win = false;
    for v in e[u].iter().copied() {
        dfs(e, r, used, v);

        if r[v] == 2 {
            r[u] = 1; // win
            win = true;
        }
    }
    if r[u] == 3 && win {
        r[u] = 1;
    }
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let mut s2 = vec![(n, n); n];
    let mut map = BTreeMap::new();
    for (i, s_i) in s.iter().enumerate() {
        let head = &s_i[0..3];
        let tail = &s_i[s_i.len() - 3..s_i.len()];
        let len = map.len();
        let eh = *map.entry(head).or_insert(len);
        let len = map.len();
        let et = *map.entry(tail).or_insert(len);
        s2[i] = (eh, et);
    }
    let v_len = map.len();
    let mut v = vec![String::new(); v_len];
    for (k, j) in map {
        v[j] = k.iter().collect::<String>();
    }

    let e = adjacency_list(v_len, &s2);

    let mut r = vec![0; v_len];
    let mut used = vec![0; v_len];
    dfs(&e, &mut r, &mut used, 0);
    // eprintln!("s2 = {:?}", s2);
    // eprintln!("v  = {:?}", v);
    // eprintln!("e  = {:?}", e);

    for i in 0..n {
        let u = s2[i].1;
        println!(
            "{}",
            match r[u] {
                1 => "Aoki",
                2 => "Takahashi",
                _ => "Draw",
            }
        );
    }
}
