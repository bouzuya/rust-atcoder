use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut e = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        e[a].push(b);
        e[b].push(a);
    }

    let mut ans = vec![n; n];
    let mut sp = vec![n; n];
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(std::cmp::Reverse((0, 0, 0)));
    while let Some(std::cmp::Reverse((c, u, p))) = pq.pop() {
        if c > sp[u] {
            continue;
        }
        sp[u] = c;
        ans[u] = p;
        for &v in e[u].iter() {
            if c + 1 < sp[v] {
                sp[v] = c + 1;
                ans[v] = u;
                pq.push(std::cmp::Reverse((c + 1, v, u)));
            }
        }
    }

    println!("Yes");
    for i in 1..n {
        println!("{}", ans[i] + 1);
    }
}
