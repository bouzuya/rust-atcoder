use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        p: usize,
        h: [usize; n],
    }
    let c = std::iter::once(0)
        .chain(h.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    let modp = 1234567;

    let mut sum = vec![0_usize; n + 1 + 1];
    let mut dp = vec![0_usize; n + 1];
    dp[0] = 1_usize;
    sum[1] = 1_usize;
    for i in 1..=n {
        let v_l = c[i].saturating_sub(p);
        let left = c.lower_bound(&v_l);
        let right = i;
        // for j in left..right {
        //     dp[i] += dp[j];
        //     dp[i] %= modp;
        // }
        dp[i] += (modp + sum[right] - sum[left]) % modp;
        sum[i + 1] += sum[i] + dp[i];
        sum[i + 1] %= modp;
    }
    let ans = dp[n];
    println!("{}", ans);
}
