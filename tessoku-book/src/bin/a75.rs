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
        mut td: [(usize, usize); n],
    };
    td.sort_by_key(|&(_, d)| d);

    let mut dp = vec![vec![0; 1440 + 1]; n + 1];
    for (i, (t, d)) in td.iter().copied().enumerate() {
        for j in 0..=1440 {
            if j + t <= d && j + t <= 1440 {
                chmax!(dp[i + 1][j + t], dp[i][j] + 1);
            }
            chmax!(dp[i + 1][j], dp[i][j]);
        }
    }
    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}
