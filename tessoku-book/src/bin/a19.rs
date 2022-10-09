use proconio::input;

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
        n: usize,
        capital_w: usize,
        wv: [(usize, usize); n],
    };

    let mut dp = vec![vec![0_usize; capital_w + 1]; n + 1];
    for (i, (w, v)) in wv.iter().copied().enumerate() {
        for j in 0..=capital_w {
            chmax!(dp[i + 1][j], dp[i][j]);
            if j + w <= capital_w {
                chmax!(dp[i + 1][j + w], dp[i][j] + v);
            }
        }
    }
    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}
