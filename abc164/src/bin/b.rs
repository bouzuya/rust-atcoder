use proconio::input;

fn f(h: i64, s: i64) -> i64 {
    (h + s - 1) / s
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };
    let ans = if f(c, b) <= f(a, d) { "Yes" } else { "No" };
    println!("{}", ans);
}
