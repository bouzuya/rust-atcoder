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

fn f(n: usize, inf: usize, e: &Vec<Vec<usize>>, s: usize) -> Vec<(usize, usize)> {
    let mod_p = 1_000_000_007;
    let mut d = vec![(inf, 0); n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = (0, 1);
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u].0 {
            continue;
        }
        for &v in e[u].iter() {
            let w = w_u + 1;
            if w < d[v].0 {
                d[v] = (w, d[u].1);
                pq.push(std::cmp::Reverse((w, v)));
            } else if w == d[v].0 {
                d[v].1 += d[u].1;
                d[v].1 %= mod_p;
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        a: Usize1,
        b: Usize1,
        m: usize,
        xy: [(Usize1, Usize1); m],
    };

    let e = adjacency_list(n, &xy);
    let inf = 1_000_000_000;
    let d = f(n, inf, &e, a);
    let ans = d[b].1;
    println!("{}", ans);
}
