use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dijkstra(n: usize, inf: u64, e: &Vec<Vec<(usize)>>, s: usize) -> (Vec<u64>, Vec<usize>) {
    let mut count = vec![0; n];
    let mut d = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    count[s] = 1;
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
                count[v] = count[u];
                pq.push(std::cmp::Reverse((w, v)));
            } else if w == d[v] {
                count[v] += count[u];
                count[v] %= 1_000_000_007;
            }
        }
    }
    (d, count)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let e = adjacency_list(n, &ab);
    let inf = 1_000_000_000_000_000_000;
    let (_, count) = dijkstra(n, inf, &e, 0);

    let ans = count[n - 1];
    println!("{}", ans);
}
