use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvc: [(usize, usize, usize); m],
    }

    let inf = 1_usize << 60;
    let mut dist = vec![vec![inf; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for (u, v, c) in uvc {
        dist[u][v] = dist[u][v].min(c);
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]).min(inf);
            }
        }
    }
    let mut sum = 0_usize;
    for i in 0..n {
        for j in 0..n {
            sum += dist[i][j];
        }
    }
    let ans = sum;
    println!("{}", ans);
}
