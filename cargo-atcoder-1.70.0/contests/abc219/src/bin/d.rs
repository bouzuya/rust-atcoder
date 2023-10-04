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
        x: usize,
        y: usize,
        ab: [(usize, usize); n],
    }

    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; y + 1]; x + 1];
    dp[0][0] = 0;
    for (a, b) in ab {
        let mut next = vec![vec![inf; y + 1]; x + 1];
        for i in 0..=x {
            for j in 0..=y {
                chmin!(next[i][j], dp[i][j]);
                chmin!(next[(i + a).min(x)][(j + b).min(y)], dp[i][j] + 1);
            }
        }
        dp = next;
    }

    let ans = dp[x][y];
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
