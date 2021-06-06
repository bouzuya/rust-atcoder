use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
    }
    e
}

fn dijkstra(n: usize, inf: u64, e: &Vec<Vec<usize>>, s: usize) -> Vec<u64> {
    let mut d = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for &v in e[u].iter() {
            let w = w_u + 1;
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
        ab: [(Usize1, Usize1); m],
    };

    let e = adjacency_list(n, &ab);
    let inf = 1_000_000_000;
    let mut count = 0_usize;
    for u in 0..n {
        let d = dijkstra(n, inf, &e, u);
        count += d.iter().filter(|&&d_i| d_i != inf).count();
    }
    let ans = count;
    println!("{}", ans);
}
