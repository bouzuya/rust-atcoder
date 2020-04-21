use proconio::input;

fn main() {
    input! {
        n: usize,
        t: i64,
        mut abv: [(i64, i64); n],
    };
    abv.sort_by_key(|(a, b)| (-b, -a));
    let mut dp = vec![vec![0_i64; (t as usize - 1) + 1]; (n - 1) + 1];
    dp[0][0] = 0;
    for i in 1..=(n - 1) {
        dp[i][0] = 0;
        for j in 1..=(t as usize - 1) {
            let (a, b) = abv[i];
            dp[i][j] = std::cmp::max(
                dp[i - 1][j],
                std::cmp::max(
                    dp[i][j - 1],
                    if j as i64 >= a {
                        dp[i - 1][j - a as usize] + b
                    } else {
                        0
                    },
                ),
            );
        }
    }
    let ans = abv[0].1 + dp[n - 1][t as usize - 1];
    println!("{}", ans);
}
