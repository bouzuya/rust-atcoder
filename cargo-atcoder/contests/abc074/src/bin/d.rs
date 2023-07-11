use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n]
    };
    let mut d = a.clone();
    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                if d[u][k] + d[k][v] < d[u][v] {
                    d[u][v] = d[u][k] + d[k][v];
                }
            }
        }
    }
    if a != d {
        println!("-1");
        return;
    }

    let mut edges = vec![];
    for u in 0..n {
        for v in 0..n {
            edges.push((d[u][v], u, v));
        }
    }
    edges.sort();
    edges.reverse();

    let mut set = HashSet::new();
    for (i, (dist, u, v)) in edges.iter().copied().enumerate() {
        for k in 0..n {
            if k == u || k == v {
                continue;
            }
            if dist == d[u][k] + d[k][v] {
                set.insert(i);
            }
        }
    }

    let mut sum = 0_usize;
    for (i, (dist, _, _)) in edges.iter().copied().enumerate() {
        if set.contains(&i) {
            continue;
        }
        sum += dist;
    }
    let ans = sum / 2;
    println!("{}", ans);
}
