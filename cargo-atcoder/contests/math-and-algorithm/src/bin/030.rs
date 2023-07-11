use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    };
    let mut dp = vec![0; w + 1];
    dp[0] = 0;
    for (w_i, v_i) in wv {
        let mut next = vec![0; w + 1];
        for j in 0..=w {
            next[j] = next[j].max(dp[j]);
            if j + w_i <= w {
                next[j + w_i] = next[j + w_i].max(dp[j] + v_i);
            }
        }
        dp = next;
    }
    let ans = dp[w];
    println!("{}", ans);
}
