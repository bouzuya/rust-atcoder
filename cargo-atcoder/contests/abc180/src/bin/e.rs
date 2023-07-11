use proconio::input;

macro_rules! chmin {
    ($min_v: expr, $v: expr) => {
        if $v < $min_v {
            $min_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        xyz: [(i64, i64, i64); n],
    };

    let inf = 1 << 50;
    let mut edges = vec![vec![inf; n]; n];
    for (i, (x_i, y_i, z_i)) in xyz.iter().copied().enumerate() {
        for (j, (x_j, y_j, z_j)) in xyz.iter().copied().enumerate() {
            edges[i][j] = (x_i - x_j).abs() + (y_i - y_j).abs() + (z_j - z_i).max(0);
        }
    }

    let mut dp = vec![vec![inf; 1 << n]; n];
    dp[0][0] = 0;
    for i in 0..1 << n {
        for u in 0..n {
            for v in 0..n {
                chmin!(dp[v][i | (1 << v)], dp[u][i] + edges[u][v]);
            }
        }
    }
    let ans = dp[0][(1 << n) - 1];
    println!("{}", ans);
}
