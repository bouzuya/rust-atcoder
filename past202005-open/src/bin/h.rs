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
        l: usize,
        x: [usize; n],
        t_1: i64,
        t_2: i64,
        t_3: i64,
    };

    let h_1 = t_1 / 2;
    let h_2 = t_2 / 2;
    let h_3 = t_3 / 2;

    let inf = 1_000_000_000;
    let mut xp = vec![false; l + 4 + 1];
    for &x_i in x.iter() {
        xp[x_i] = true;
    }
    let mut dp = vec![inf; l + 1];
    dp[0] = 0;
    for i in 0..l {
        chmin!(
            dp[i + 1],
            dp[i] + t_1 + if xp[i] { h_3 } else { 0 } + if xp[i + 1] { h_3 } else { 0 }
        );
        chmin!(
            dp[std::cmp::min(i + 2, l)],
            dp[i]
                + h_1
                + if xp[i] { h_3 } else { 0 }
                + h_2
                + if i + 2 <= l { h_2 } else { 0 }
                + if i + 2 <= l {
                    h_1 + if xp[i + 2] { h_3 } else { 0 }
                } else {
                    0
                }
        );
        chmin!(
            dp[std::cmp::min(i + 4, l)],
            dp[i]
                + h_1
                + if xp[i] { h_3 } else { 0 }
                + h_2
                + if i + 2 <= l { h_2 } else { 0 }
                + if i + 2 <= l { h_2 } else { 0 }
                + if i + 3 <= l { h_2 } else { 0 }
                + if i + 3 <= l { h_2 } else { 0 }
                + if i + 4 <= l { h_2 } else { 0 }
                + if i + 4 <= l {
                    h_1 + if xp[i + 4] { h_3 } else { 0 }
                } else {
                    0
                }
        );
    }
    let ans = dp[l];
    println!("{}", ans);
}
