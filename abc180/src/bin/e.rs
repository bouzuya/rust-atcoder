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

    let inf = 1_000_000_000;
    let mut e = vec![vec![inf; n]; n];
    for (i, (x_i, y_i, z_i)) in xyz.iter().enumerate() {
        for (j, (x_j, y_j, z_j)) in xyz.iter().enumerate() {
            let d = (x_j - x_i).abs() + (y_j - y_i).abs() + std::cmp::max(0, z_j - z_i);
            e[i][j] = d;
        }
    }

    let mut dp = vec![vec![inf; n]; 1 << n];
    dp[0][0] = 0;

    for s in 0..1 << n {
        for v in 0..n {
            for u in 0..n {
                if (s & (1 << v)) == 0 {
                    if v != u {
                        chmin!(dp[s | (1 << v)][v], dp[s][u] + e[u][v]);
                    }
                }
            }
        }
    }

    println!("{}", dp[(1 << n) - 1][0]);
}
