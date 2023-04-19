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
        capital_n: usize,
        capital_w: usize,
        wv: [(usize, usize); capital_n],
    }

    let mut dp = vec![vec![0; capital_w + 1]; capital_n + 1];
    for (i, (w, v)) in wv.iter().copied().enumerate() {
        for j in 0..=capital_w {
            chmax!(
                dp[i + 1][j],
                dp[i][j].max(if j < w { 0 } else { dp[i][j - w] + v })
            );
        }
    }
    let ans = dp[capital_n].iter().copied().max().unwrap();
    println!("{}", ans);
}
