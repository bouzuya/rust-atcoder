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
        w: usize,
        wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];
    for (i, (w_i, v_i)) in wv.iter().copied().enumerate() {
        for j in 0..=w {
            dp[i + 1][j] = dp[i][j];
            if j >= w_i {
                chmax!(dp[i + 1][j], dp[i][j - w_i] + v_i);
            }
        }
    }
    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}
