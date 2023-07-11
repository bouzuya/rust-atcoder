use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        abc: [(Usize1, Usize1, usize); m],
        e: [Usize1; k],
    };
    let inf = 1_usize << 60;
    let mut dist = vec![inf; n];
    dist[0] = 0_usize;

    for e_i in e.iter().copied() {
        let (a, b, c) = abc[e_i];
        if dist[a] == inf {
            continue;
        }

        dist[b] = dist[b].min(dist[a] + c);
    }

    let ans = if dist[n - 1] == inf {
        -1
    } else {
        dist[n - 1] as i64
    };
    println!("{}", ans);
}
