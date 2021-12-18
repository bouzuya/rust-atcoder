use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
    }
    e
}

fn dijkstra(n: usize, inf: u64, e: &[Vec<usize>], s: usize) -> Vec<Vec<u64>> {
    let mut d = vec![vec![inf; 3]; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s][0] = 0;
    pq.push(std::cmp::Reverse((0, s, 0)));
    while let Some(std::cmp::Reverse((d_u, u, r_u))) = pq.pop() {
        if d_u > d[u][r_u] {
            continue;
        }
        for v in e[u].iter().copied() {
            let d_v = d_u + 1;
            let r_v = (r_u + 1) % 3;
            if d_v < d[v][r_v] {
                d[v][r_v] = d_v;
                pq.push(std::cmp::Reverse((d_v, v, r_v)));
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
        s: Usize1,
        t: Usize1,
    };

    let e = adjacency_list(n, &uv);
    let inf = 1_000_000_000;
    let d = dijkstra(n, inf, &e, s);
    let d = d[t][0] as i64;
    let ans = if d == (inf as i64) {
        -1
    } else {
        (d + 3 - 1) / 3
    };
    println!("{}", ans);
}
