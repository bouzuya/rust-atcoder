use proconio::input;

fn dijkstra(n: usize, inf: u64, e: &Vec<Vec<(usize, u64)>>, s: usize) -> Vec<u64> {
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
        h: usize,
        w: usize,
        c: [[u64; 10]; 10],
        a: [[i64; w]; h],
    };
    let mut e = vec![vec![]; 10];
    for (i, c_i) in c.iter().enumerate() {
        for (j, &c_ij) in c_i.iter().enumerate() {
            if i == j {
                continue;
            }
            e[i].push((j, c_ij));
        }
    }
    let inf = 1_000_000_u64;
    let mut d = vec![];
    for i in 0..10 {
        d.push(dijkstra(10, inf, &e, i));
    }
    let mut ans = 0;
    for a_i in a.iter() {
        for &a_ij in a_i.iter() {
            match a_ij {
                -1 | 1 => {} // do nothing
                0 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => ans += d[a_ij as usize][1],
                _ => unreachable!(),
            }
        }
    }
    println!("{}", ans);
}
