use proconio::input;
use proconio::marker::{Chars, Usize1};

fn adjacency_list(n: usize, uvw: &Vec<(usize, usize, u64)>) -> Vec<Vec<(usize, u64)>> {
    let mut e = vec![vec![]; n];
    for &(u, v, w) in uvw.iter() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn dijkstra(n: usize, inf: u64, e: &Vec<Vec<(usize, u64)>>, s: usize) -> Vec<u64> {
    let mut d = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for &(v, w_v) in e[u].iter() {
            let w = w_u + w_v;
            if w < d[v] {
                d[v] = w;
                pq.push(std::cmp::Reverse((w, v)));
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        x_ab: u64,
        x_ac: u64,
        x_bc: u64,
        s: Chars,
        abc: [(Usize1, Usize1, u64); m],
    };
    let inf = 1_000_000_000_000_000_000;
    let mut e = adjacency_list(n, &abc);
    let (a_i, b_i, c_i) = (n + 0, n + 1, n + 2);
    let (a_o, b_o, c_o) = (n + 3, n + 4, n + 5);
    for _ in 0..6 {
        e.push(vec![]);
    }
    for c in "ABC".chars() {
        for i in s
            .iter()
            .enumerate()
            .filter(|(_, &s_i)| s_i == c)
            .map(|(i, _)| i)
        {
            match c {
                'A' => {
                    e[i].push((a_i, 0));
                    e[a_o].push((i, 0));
                }
                'B' => {
                    e[i].push((b_i, 0));
                    e[b_o].push((i, 0));
                }
                'C' => {
                    e[i].push((c_i, 0));
                    e[c_o].push((i, 0));
                }
                _ => unreachable!(),
            }
        }
    }
    e[a_i].push((b_o, x_ab));
    e[a_i].push((c_o, x_ac));
    e[b_i].push((a_o, x_ab));
    e[b_i].push((c_o, x_bc));
    e[c_i].push((a_o, x_ac));
    e[c_i].push((b_o, x_bc));
    let d = dijkstra(n + 6, inf, &e, 0);
    let ans = d[n - 1];
    println!("{}", ans);
}
