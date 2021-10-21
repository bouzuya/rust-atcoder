use proconio::input;

fn f(a: &[i64], dp: &mut [Vec<i64>], b: &mut [Vec<bool>], l: usize, r: usize) -> i64 {
    if b[l][r] {
        return dp[l][r];
    }
    b[l][r] = true;
    let x_minus_y = if l < r {
        (a[l] - f(a, dp, b, l + 1, r)).max(a[r] - f(a, dp, b, l, r - 1))
    } else {
        a[l]
    };
    dp[l][r] = x_minus_y;
    x_minus_y
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut b = vec![vec![false; n]; n];
    let mut dp = vec![vec![0; n]; n];
    let ans = f(&a, &mut dp, &mut b, 0, n - 1);
    println!("{}", ans);
}
