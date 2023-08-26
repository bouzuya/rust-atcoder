use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, u64); m],
    };

    let inf = 1_u64 << 60;
    let mut edges = vec![vec![inf; n]; n];
    for (a, b, c) in abc {
        edges[a][b] = c;
        edges[b][a] = c;
    }

    let inf = 1_u64 << 60;

    let mut is = (0..n).collect::<Vec<usize>>();
    let mut max = 0_u64;
    loop {
        let mut sum = 0_u64;
        let mut u = is[0];
        for v in is.iter().copied().skip(1) {
            if edges[u][v] != inf {
                sum += edges[u][v];
                u = v;
            } else {
                break;
            }
        }
        max = max.max(sum);
        if !is.next_permutation() {
            break;
        }
    }
    let ans = max;
    println!("{}", ans);
}
