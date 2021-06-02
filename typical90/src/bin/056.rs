use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    };
    let max_s = s;
    let mut dp = vec![vec![2; max_s + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (a_i, b_i) = ab[i];
        for s_i in 0..=max_s {
            if dp[i][s_i] == 2 {
                continue;
            }
            if s_i + b_i <= max_s {
                dp[i + 1][s_i + b_i] = 1;
            }
            if s_i + a_i <= max_s {
                dp[i + 1][s_i + a_i] = 0;
            }
        }
    }
    if dp[n][s] == 2 {
        println!("Impossible");
        return;
    }
    let mut s_n = s;
    let mut g_n = dp[n][s];
    let mut ans = vec![g_n];
    for i in (1..n).rev() {
        let s_p = s_n - if g_n == 0 { ab[i].0 } else { ab[i].1 };
        let g_p = dp[i][s_p];
        ans.push(g_p);
        s_n = s_p;
        g_n = g_p;
    }

    ans.reverse();
    for a in ans {
        print!("{}", if a == 0 { 'A' } else { 'B' });
    }
    println!();
}
