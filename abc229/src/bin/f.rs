use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    };
    let inf = 1_000_000_000_000_000_000_usize;
    let mut ans = inf;
    for c_start in 0..2 {
        let mut dp = vec![inf; 2];
        dp[c_start] = 0;
        for i in 0..n {
            let a_i = a[i];
            let b_i_1 = b[(i + n - 1) % n];
            let mut next = vec![inf; 2];
            next[0] = next[0].min(dp[0] + a_i + b_i_1);
            next[1] = next[1].min(dp[0]);
            next[0] = next[0].min(dp[1] + a_i);
            next[1] = next[1].min(dp[1] + b_i_1);
            dp = next;
        }
        ans = ans.min(dp[c_start]);
    }
    println!("{}", ans);
}
