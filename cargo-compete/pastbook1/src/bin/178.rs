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
        t: [usize; 3],
    }
    let mut y = vec![false; 2 * l + 8 + 1];
    for x_i in x {
        y[2 * x_i] = true;
    }
    let (t1, t2, t3) = (t[0] / 2, t[1] / 2, t[2]);
    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; 2]; 2 * l + 8 + 1];
    dp[0][0] = 0_usize;
    for i in (0..=2 * l).step_by(2) {
        chmin!(
            dp[i + 2][0],
            dp[i][0] + t1 + t1 + if y[i + 2] { t3 } else { 0 }
        );
        chmin!(
            dp[i + 4][0],
            dp[i][0] + t1 + t2 * 2 + t1 + if y[i + 4] { t3 } else { 0 }
        );
        chmin!(
            dp[i + 8][0],
            dp[i][0] + t1 + t2 * 6 + t1 + if y[i + 8] { t3 } else { 0 }
        );
        chmin!(dp[i + 2][1], dp[i][0] + t1 + t2);
        chmin!(dp[i + 3][1], dp[i][0] + t1 + t2 * 2);
        chmin!(dp[i + 4][1], dp[i][0] + t1 + t2 * 3);
        chmin!(dp[i + 5][1], dp[i][0] + t1 + t2 * 4);
        chmin!(dp[i + 6][1], dp[i][0] + t1 + t2 * 5);
        chmin!(dp[i + 7][1], dp[i][0] + t1 + t2 * 6);
    }
    let ans = *dp[2 * l].iter().min().unwrap();
    println!("{}", ans);
}
