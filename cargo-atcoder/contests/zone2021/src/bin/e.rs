use proconio::input;

fn dijkstra(n: usize, inf: usize, e: &[Vec<(usize, usize)>], s: usize) -> Vec<usize> {
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
        r: usize,
        c: usize,
        a: [[usize; c - 1]; r],
        b: [[usize; c]; r - 1],
    };
    let n = r * c * 2;
    let mut e = vec![vec![]; n];
    for (i, a_i) in a.iter().enumerate() {
        for (j, a_ij) in a_i.iter().copied().enumerate() {
            let (u, v) = (i * c + j, i * c + j + 1);
            e[u].push((v, a_ij));
            e[v].push((u, a_ij));
        }
    }
    for (i, b_i) in b.iter().enumerate() {
        for (j, b_ij) in b_i.iter().copied().enumerate() {
            let (u, v) = (i * c + j, (i + 1) * c + j);
            e[u].push((v, b_ij));
        }
    }
    for i in 0..r {
        for j in 0..c {
            let offset = r * c;
            let (u1, v1) = (i * c + j, (i + 1) * c + j);
            let (u2, v2) = (u1 + offset, v1 + offset);
            e[u1].push((u2, 1));
            e[u2].push((u1, 0));
            if i + 1 < r {
                e[v2].push((u2, 1));
            }
        }
    }

    let inf = 1 << 62;
    let d = dijkstra(n, inf, &e, 0);

    let ans = d[r * c - 1];
    println!("{}", ans);
}
