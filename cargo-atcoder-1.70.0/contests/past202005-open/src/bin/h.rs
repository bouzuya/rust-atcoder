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
    };
    let l = l * 2;
    let x = x.into_iter().map(|x| x * 2).collect::<Vec<usize>>();
    let h = {
        let mut y = vec![false; l + 8 + 1];
        for x_i in x {
            y[x_i] = true;
        }
        y
    };
    let (t1, t2, t3) = (t[0] / 2, t[1] / 2, t[2]);

    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; 2]; l + 8 + 1];
    dp[0][0] = 0_usize;
    for i in (0..l).step_by(2) {
        chmin!(
            dp[i + 2][0],
            dp[i][0] + t1 * 2 + if h[i + 2] { t3 } else { 0 }
        );
        chmin!(
            dp[i + 1 + 2 + 1][0],
            dp[i][0] + t1 + t2 * 2 + t1 + if h[i + 1 + 2 + 1] { t3 } else { 0 }
        );
        chmin!(
            dp[i + 1 + 6 + 1][0],
            dp[i][0] + t1 + t2 * 6 + t1 + if h[i + 1 + 6 + 1] { t3 } else { 0 }
        );
        for j in 1..=6 {
            chmin!(dp[i + 1 + j][1], dp[i][0] + t1 + t2 * j);
        }
    }
    let ans = dp[l][0].min(dp[l][1]);
    println!("{}", ans);
}
