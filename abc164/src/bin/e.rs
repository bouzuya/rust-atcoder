use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        uvabs: [(Usize1, Usize1, usize, usize); m],
        cds: [(usize, usize); n],
    };

    let mut edges = vec![vec![]; n];
    let mut max_c = 0_usize;
    for (u, v, c, d) in uvabs {
        edges[u].push((v, c, d));
        edges[v].push((u, c, d));
        max_c = std::cmp::max(max_c, c);
    }
    let max_s = max_c * (n - 1);
    let inf = 1_000_000_000_000_000_000_usize;

    let mut dp = vec![vec![inf; max_s + 1]; n];
    dp[0][std::cmp::min(s, max_s)] = 0_usize;
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(std::cmp::Reverse((0_usize, 0, std::cmp::min(s, max_s))));
    while let Some(std::cmp::Reverse((t, u, s))) = pq.pop() {
        let (c, d) = cds[u];
        if s + c <= max_s {
            let (nv, nt, ns) = (u, t + d, s + c);
            if nt < dp[nv][ns] {
                dp[nv][ns] = nt;
                pq.push(std::cmp::Reverse((nt, nv, ns)));
            }
        }
        for &(v, c, d) in edges[u].iter() {
            if c <= s {
                let (nv, nt, ns) = (v, t + d, s - c);
                if nt < dp[nv][ns] {
                    dp[nv][ns] = nt;
                    pq.push(std::cmp::Reverse((nt, nv, ns)));
                }
            }
        }
    }
    for cs in dp.iter().skip(1) {
        let c = cs.iter().min().unwrap();
        println!("{}", c);
    }
}
