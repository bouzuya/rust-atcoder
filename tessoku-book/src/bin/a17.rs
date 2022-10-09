use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 2],
    };
    let inf = 1 << 60;
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = (dp[i - 1] + a[i - 1]).min(if i > 1 { dp[i - 2] + b[i - 2] } else { inf })
    }

    let mut ans = vec![n - 1];
    let mut cur = n - 1;
    while cur > 0 {
        if dp[cur - 1] + a[cur - 1] == dp[cur] {
            ans.push(cur - 1);
            cur = cur.saturating_sub(1);
        } else {
            ans.push(cur - 2);
            cur = cur.saturating_sub(2);
        }
    }
    ans.reverse();

    println!("{}", ans.len());
    for a in ans {
        println!("{}", a + 1);
    }
}
