use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xdv: [(i64, i64); n],
    };
    xdv.sort();

    let mod_p = 998244353;
    let mut dp = vec![0usize; n + 1];
    dp[n] = 1;
    for i in (0..n).rev() {
        let j = f(&xdv, i);
        dp[i] = (dp[i + 1] + dp[j + 1]) % mod_p;
    }

    let ans = dp[0];
    println!("{}", ans);
}

fn f(xdv: &Vec<(i64, i64)>, i: usize) -> usize {
    let r_x = xdv[i].0 + xdv[i].1;
    let mut r_i = i;
    for j in i + 1..xdv.len() {
        if xdv[j].0 < r_x {
            r_i = std::cmp::max(r_i, f(xdv, j));
        }
    }
    r_i
}
