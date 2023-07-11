use proconio::input;

fn is_ok(n: usize, k: usize, a: &Vec<usize>, i_s: usize) -> bool {
    let a_s = a[i_s];
    let mut dp = vec![vec![false; k]; n];
    dp[0][0] = true;
    for i in 0..n {
        if i == i_s {
            continue;
        }
        let a_i = a[i];
        let i = if i > i_s { i - 1 } else { i };
        for j in 0..k {
            dp[i + 1][j] |= dp[i][j];
            if j + a_i < k {
                dp[i + 1][j + a_i] |= dp[i][j];
            }
        }
    }
    !dp[n - 1][k.saturating_sub(a_s)..k].iter().any(|&b_i| b_i)
}

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    };
    a.sort();
    let mut l = 0;
    let mut r = n;
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if is_ok(n, k, &a, m) {
            l = m;
        } else {
            r = m;
        }
    }
    if is_ok(n, k, &a, 0) {
        let ans = l + 1;
        println!("{}", ans);
    } else {
        println!("{}", 0);
    }
}
