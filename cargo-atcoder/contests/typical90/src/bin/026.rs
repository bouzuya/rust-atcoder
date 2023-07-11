use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
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
        ab: [(Usize1, Usize1); n - 1],
    };

    let e = adjacency_list(n, &ab);
    let inf = 1_000_000_000;
    let d = dijkstra(n, inf, &e, 0);
    let mut es = vec![];
    let mut os = vec![];
    for (v, &d_i) in d.iter().enumerate() {
        if d_i % 2 == 0 {
            es.push(v);
        } else {
            os.push(v);
        }
    }
    let vs = if es.len() >= n / 2 { es } else { os };
    let ans = vs
        .iter()
        .take(n / 2)
        .map(|&v| format!("{}", v + 1))
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
