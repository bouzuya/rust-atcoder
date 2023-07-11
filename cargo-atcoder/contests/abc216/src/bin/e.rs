use proconio::input;

fn arithmetic_series(a_1: i64, a_n: i64, n: i64) -> i64 {
    (a_1 + a_n) * n / 2
}

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    };
    let f = |x: i64| -> i64 { a.iter().copied().map(|a_i| (a_i - x).max(0)).sum::<i64>() };

    // f(x) < k を満たす最大の x を求める
    let mut l = -1_i64;
    let mut r = 1_000_000_000_000_i64;
    while l + 1 < r {
        let m = (l + r) / 2;
        if f(m) < k {
            r = m;
        } else {
            l = m;
        }
    }
    let x = r;
    let mut sum = 0;
    for a_i in a.iter().copied() {
        if a_i <= x {
            continue;
        }
        sum += arithmetic_series(x + 1, a_i, a_i - (x + 1) + 1);
    }
    sum += x * (k - f(x));
    let ans = sum;
    println!("{}", ans);
}
