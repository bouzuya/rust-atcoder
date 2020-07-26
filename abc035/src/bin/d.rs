use proconio::input;
use proconio::marker::Usize1;

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
        t: u64,
        a: [u64; n],
        abc: [(Usize1, Usize1, u64); m],
    };
    let inf = 1_000_000_000_000_000_u64;

    let mut to = vec![vec![]; n];
    for &(a_i, b_i, c_i) in abc.iter() {
        to[a_i].push((b_i, c_i));
    }
    let mut from = vec![vec![]; n];
    for &(a_i, b_i, c_i) in abc.iter() {
        from[b_i].push((a_i, c_i));
    }

    let mut ans = 0;
    let d_to = dijkstra(n, inf, &to, 0);
    let d_from = dijkstra(n, inf, &from, 0);
    for (i, (&to, &from)) in d_to.iter().zip(d_from.iter()).enumerate() {
        if to != inf && from != inf && to + from < t {
            ans = std::cmp::max(ans, a[i] * (t - (to + from)));
        }
    }
    println!("{}", ans);
}
