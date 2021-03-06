use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, ab: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        e[a].push(b);
        e[b].push(a);
    }
    e
}

fn shortest_path(e: &Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let n = e.len();
    let inf = n;
    let mut d = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((c_u, u))) = pq.pop() {
        for &v in e[u].iter() {
            let c_v = c_u + 1;
            if c_v < d[v] {
                d[v] = c_v;
                pq.push(std::cmp::Reverse((c_v, v)));
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
    let sp = shortest_path(&e, 0);
    let ans = if sp[n - 1] <= 2 {
        "POSSIBLE"
    } else {
        "IMPOSSIBLE"
    };
    println!("{}", ans);
}
