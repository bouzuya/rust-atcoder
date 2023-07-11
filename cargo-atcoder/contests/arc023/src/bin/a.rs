use proconio::input;

fn f(y: i64, m: i64, d: i64) -> i64 {
    let y = match m {
        1 | 2 => y - 1,
        _ => y,
    };
    let m = match m {
        1 => 13,
        2 => 14,
        _ => m,
    };
    365 * y + (y / 4) - (y / 100) + (y / 400) + ((306 * (m + 1)) / 10) + d - 429
}

fn main() {
    input! {
        y: i64,
        m: i64,
        d: i64,
    };
    let ans = f(2014, 5, 17) - f(y, m, d);
    println!("{}", ans);
}
