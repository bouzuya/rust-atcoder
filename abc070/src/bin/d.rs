use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        abc: [(Usize1, Usize1, u64); n - 1],
        q: usize,
        k: Usize1,
        xy: [(Usize1, Usize1); q],
    };

    let mut e = vec![vec![]; n];
    for &(a, b, c) in abc.iter() {
        e[a].push((c, b));
        e[b].push((c, a));
    }

    // K から各頂点への最短距離 O(N)
    let inf = 1_000_000_000_000_000_u64;
    let mut sp = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    pq.push((0, k));
    while let Some((c, u)) = pq.pop() {
        if c > sp[u] {
            continue;
        }
        sp[u] = c;
        for &(c_v, v) in e[u].iter() {
            pq.push((c + c_v, v));
        }
    }

    for &(x, y) in xy.iter() {
        let ans = sp[x] + sp[y];
        println!("{}", ans);
    }
}
