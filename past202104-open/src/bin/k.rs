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
        pu: [(i64, i64); n],
    };
    let inf = 1_000_000_000_i64;
    let mut dp = vec![vec![-inf; 100 + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (p_i, u_i) = pu[i];
        let s_i = u_i - p_i + p_i / 100 * 20;
        for j in 0..100 {
            chmax!(dp[i + 1][j], dp[i][j]);
            if j + p_i as usize % 100 >= 100 {
                chmax!(dp[i + 1][(j + p_i as usize) % 100], dp[i][j] + s_i + 20);
            } else {
                chmax!(dp[i + 1][j + p_i as usize % 100], dp[i][j] + s_i);
            }
        }
    }
    let ans = *dp[n].iter().max().unwrap();
    println!("{}", ans);
}
