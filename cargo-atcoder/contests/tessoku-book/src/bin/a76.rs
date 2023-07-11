use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        w: i64,
        l: i64,
        r: i64,
        x: [i64; n],
    };

    let x = {
        let mut a = vec![];
        a.push(0);
        a.extend(x);
        a.push(w);
        a
    };
    let n = x.len();

    let p = 1_000_000_007;

    let mut dp = vec![0_usize; n + 1];
    let mut sum = vec![0_usize; n + 1];
    dp[0] = 1;
    sum[0] = 0;
    sum[1] = 1;
    for (i, x_i) in x.iter().copied().enumerate() {
        if x_i < l {
            sum[i + 2] = sum[i + 1];
            continue;
        }
        let il = x.lower_bound(&(x_i - r));
        let ir = x.lower_bound(&(x_i - l + 1)) - 1;
        dp[i] = (p + sum[ir + 1] - sum[il]) % p;
        sum[i + 1] = sum[i] + dp[i];
    }

    let ans = dp[n - 1];
    println!("{}", ans);
}
