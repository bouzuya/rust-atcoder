use proconio::input;
use proconio::marker::Chars;

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
        m: usize,
        sc: [(Chars, i64); m]
    };
    let inf = 1_000_000_000_000_i64;
    let bc = sc
        .iter()
        .map(|(s, c)| {
            let mut bits = 0_usize;
            for i in 0..n {
                if s[i] == 'Y' {
                    bits |= 1 << (n - 1 - i);
                }
            }
            (bits, *c)
        })
        .collect::<Vec<(usize, i64)>>();
    let mut dp = vec![vec![inf; 1 << n]; m + 1];
    dp[0][0] = 0;
    for (i, &(b_i, c_i)) in bc.iter().enumerate() {
        for j in 0..1 << n {
            chmin!(dp[i + 1][j], dp[i][j]);
            chmin!(dp[i + 1][j | b_i], dp[i][j] + c_i);
        }
    }
    let mut mask = 0;
    for i in 0..n {
        mask |= 1 << (n - 1 - i);
    }
    let ans = dp[m][mask];
    println!("{}", if ans == inf { -1 } else { ans });
}
