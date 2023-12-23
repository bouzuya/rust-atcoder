use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: (f64, f64),
        xy: [(f64, f64); n],
    };

    let dist = |(x1, y1): (f64, f64), (x2, y2): (f64, f64)| -> f64 {
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    };

    let mut inf = 1_000_000_000_000_000_f64;
    let mut dp = vec![inf; k + 1];
    dp[k - 1] = dist(s, xy[n - 1]);
    for i in (0..n - 1).rev() {
        let mut dp = vec![inf; k + 1];
    }
    let ans = n - a.len();
    println!("{}", ans);
}
