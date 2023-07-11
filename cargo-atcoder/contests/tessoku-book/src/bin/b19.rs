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
        capital_w: usize,
        wv: [(usize, usize); n],
    };
    let capital_v = 1_000 * n;
    let inf = capital_w + 1;
    let mut dp = vec![vec![inf; capital_v + 1]; n + 1];
    dp[0][0] = 0;
    for (i, (w, v)) in wv.iter().copied().enumerate() {
        for j in 0..=capital_v {
            chmin!(dp[i + 1][j], dp[i][j]);
            if dp[i][j] + w <= capital_w {
                chmin!(dp[i + 1][j + v], dp[i][j] + w);
            }
        }
    }
    let ans = dp[n]
        .iter()
        .enumerate()
        .rev()
        .filter(|&(_, w)| *w <= capital_w)
        .map(|(v, _)| v)
        .next()
        .unwrap();
    println!("{}", ans);
}
