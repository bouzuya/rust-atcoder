use proconio::input;
use proconio::marker::Usize1;
use superslice::*;

fn f(inf: i64, n: usize, e: &Vec<Vec<(i64, usize)>>, u: usize) -> Vec<i64> {
    let mut sp = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(std::cmp::Reverse((0, u)));
    while let Some(std::cmp::Reverse((c_u, u))) = pq.pop() {
        if c_u < sp[u] {
            sp[u] = c_u;
            for &(c_v, v) in e[u].iter() {
                pq.push(std::cmp::Reverse((c_u + c_v, v)));
            }
        }
    }
    sp
}

fn main() {
    input! {
        n: usize,
        m: usize,
        n_r: usize,
        r: [Usize1; n_r],
        abc: [(Usize1, Usize1, i64); m],
    };
    let mut e = vec![vec![]; n];
    for &(a, b, c) in abc.iter() {
        e[a].push((c, b));
        e[b].push((c, a));
    }
    // p[i][j] 頂点 r[i] から頂点 j への最短距離
    let inf = 1_000_000_000_000_000_i64;
    let p = r
        .iter()
        .map(|&r_i| f(inf, n, &e, r_i))
        .collect::<Vec<Vec<i64>>>();
    // r の並びをすべて試して距離の合計が最小のものを選択する
    let mut ans = inf;
    let mut o = (0..r.len()).collect::<Vec<usize>>();
    loop {
        ans = std::cmp::min(
            ans,
            o.windows(2)
                .map(|w| {
                    if let &[i, j] = w {
                        p[i][r[j]]
                    } else {
                        unreachable!()
                    }
                })
                .sum::<i64>(),
        );
        if !o.next_permutation() {
            break;
        }
    }
    println!("{}", ans);
}
