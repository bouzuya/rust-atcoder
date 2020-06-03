use proconio::input;

macro_rules! chmin {
    ($e: expr, $v: expr) => {
        if $v < $e {
            $e = $v;
        }
    };
}

fn main() {
    input! {
        n: usize,
        m_a: usize,
        m_b: usize,
        abc: [(usize, usize, usize); n],
    };
    let inf = 100 * n + 1;
    let max_ab = 10 * n;
    let mut dp = vec![vec![vec![inf; max_ab + 1]; max_ab + 1]; n + 1];
    dp[0][0][0] = 0;
    for (i, &(a_i, b_i, c_i)) in abc.iter().enumerate() {
        for j in 0..=max_ab {
            for k in 0..=max_ab {
                chmin!(dp[i + 1][j][k], dp[i][j][k]);
                if j + a_i <= max_ab && k + b_i <= max_ab {
                    chmin!(dp[i + 1][j + a_i][k + b_i], dp[i][j][k] + c_i);
                }
            }
        }
    }
    let mut ans = inf;
    for i in 0..=max_ab {
        for j in 0..=max_ab {
            if (i == 0 && j == 0) || i < m_a || j < m_b || i * m_b != j * m_a {
                continue;
            }
            ans = std::cmp::min(ans, dp[n][i][j]);
        }
    }
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
