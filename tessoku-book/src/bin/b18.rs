use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    };

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..=s {
            if !dp[i][j] {
                continue;
            }
            dp[i + 1][j] = true;
            if j + a[i] <= s {
                dp[i + 1][j + a[i]] = true;
            }
        }
    }

    if !dp[n][s] {
        println!("-1");
        return;
    }

    let mut cur = s;
    let mut ans = vec![];
    for i in (0..=n).rev().take(n) {
        if !dp[i - 1][cur] {
            cur -= a[i - 1];
            ans.push(i);
        }
    }
    ans.reverse();

    println!("{}", ans.len());
    for a in ans {
        println!("{}", a);
    }
}
