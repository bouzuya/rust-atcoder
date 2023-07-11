use proconio::input;

fn dijkstra(n: usize, inf: u64, e: &[Vec<(usize, u64)>], s: usize) -> Vec<u64> {
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
        s: [u64; n],
        t: [u64; n],
    };
    let mut e = vec![vec![]; n + 1];
    for (u, s_i) in s.iter().copied().enumerate() {
        let v = if u + 1 == n { 0 } else { u + 1 };
        e[u].push((v, s_i));
    }
    for (v, t_i) in t.iter().copied().enumerate() {
        e[n].push((v, t_i));
    }
    let inf = 1_000_000_000_000_000;
    let d = dijkstra(e.len(), inf, &e, n);
    for d_i in d.iter().copied().take(n) {
        println!("{}", d_i);
    }
}
