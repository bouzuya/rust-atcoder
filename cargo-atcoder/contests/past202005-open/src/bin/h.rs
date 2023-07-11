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
        l: usize,
        x: [usize; n],
        t1: usize,
        t2: usize,
        t3: usize,
    };
    let h1 = t1 / 2;
    let h2 = t2 / 2;
    let inf = 1_000_000_000_000;
    let mut h = vec![false; 2 * l + 8 + 1];
    for &x_i in x.iter() {
        h[x_i * 2] = true;
    }
    let mut dp = vec![vec![inf; 2]; 2 * l + 8 + 1];
    dp[0][0] = 0;
    dp[0][0] = 0;
    for i in 0..2 * l {
        // act 1
        chmin!(
            dp[i + 2][0],
            dp[i][0] + h1 * 2 + if h[i + 2] { t3 } else { 0 }
        );

        // act 2
        chmin!(dp[i + 2][1], dp[i][0] + h1 + h2);
        chmin!(
            dp[i + 4][0],
            dp[i][0] + h1 * 2 + h2 * 2 + if h[i + 4] { t3 } else { 0 }
        );

        // act 3
        chmin!(dp[i + 2][1], dp[i][0] + h1 + h2);
        chmin!(dp[i + 4][1], dp[i][0] + h1 + h2 + h2 * 2);
        chmin!(dp[i + 6][1], dp[i][0] + h1 + h2 + h2 * 2 + h2 * 2);
        chmin!(
            dp[i + 8][0],
            dp[i][0] + h1 * 2 + h2 * 6 + if h[i + 8] { t3 } else { 0 }
        );
    }

    let ans = dp[2 * l].iter().min().unwrap();
    println!("{}", ans);
}
