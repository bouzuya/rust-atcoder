use proconio::input;

fn f(
    b: &mut [Vec<Vec<bool>>],
    dp: &mut [Vec<Vec<f64>>],
    n: usize,
    c1: usize,
    c2: usize,
    c3: usize,
) -> f64 {
    if b[c1][c2][c3] {
        return dp[c1][c2][c3];
    }
    b[c1][c2][c3] = true;

    if c1 + c2 + c3 == 0 {
        return 0_f64;
    }

    let p0 = (n - (c1 + c2 + c3)) as f64 / n as f64;
    let p1 = c1 as f64 / n as f64;
    let p2 = c2 as f64 / n as f64;
    let p3 = c3 as f64 / n as f64;
    let mut px = 1_f64;
    if c1 > 0 {
        px += f(b, dp, n, c1 - 1, c2, c3) * p1;
    }
    if c2 > 0 {
        px += f(b, dp, n, c1 + 1, c2 - 1, c3) * p2;
    }
    if c3 > 0 {
        px += f(b, dp, n, c1, c2 + 1, c3 - 1) * p3;
    }
    dp[c1][c2][c3] = px / (1_f64 - p0);
    dp[c1][c2][c3]
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut count = vec![0; 3 + 1];
    for a_i in a {
        count[a_i] += 1;
    }

    let mut b = vec![vec![vec![false; n + 1]; n + 1]; n + 1];
    let mut dp = vec![vec![vec![0_f64; n + 1]; n + 1]; n + 1];
    let ans = f(&mut b, &mut dp, n, count[1], count[2], count[3]);
    println!("{}", ans);
}
