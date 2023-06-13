use proconio::input;

macro_rules! chmin {
    ($min_v: expr, $v: expr) => {
        if $v < $min_v {
            $min_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let s = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();

    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; n + 1]; n + 1];
    for l in 0..n {
        let r = l + 1;
        dp[l][r] = 0;
    }
    for len in 2..=n {
        for l in 0..=n - len {
            let r = l + len;
            for m in l + 1..r {
                chmin!(dp[l][r], dp[l][m] + dp[m][r]);
            }
            dp[l][r] += s[r] - s[l];
        }
    }
    let ans = dp[0][n];
    println!("{}", ans);
}
