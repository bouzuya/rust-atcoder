use proconio::{input, marker::Usize1};

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        rcv: [(Usize1, Usize1, usize); k],
    };

    let mut d = vec![vec![0; c]; r];
    for (r_i, c_i, v_i) in rcv {
        d[r_i][c_i] = v_i;
    }

    let mut dp = vec![vec![vec![0; 3 + 1]; c + 1]; r + 1];
    for i in 0..r {
        for j in 0..c {
            for k in (0..=2).rev() {
                chmax!(dp[i][j][k + 1], dp[i][j][k] + d[i][j]);
            }
            for k in 0..=3 {
                chmax!(dp[i + 1][j][0], dp[i][j][k]);
                chmax!(dp[i][j + 1][k], dp[i][j][k]);
            }
        }
    }

    let ans = *dp[r - 1][c - 1].iter().max().unwrap();
    println!("{}", ans);
}
