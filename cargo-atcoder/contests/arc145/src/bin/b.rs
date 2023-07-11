use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    };

    let f = |x: i64| -> i64 { (x / a) * a.min(b) + (x % a).min(b - 1) };
    let ans = (f(n) - f(a - 1)).max(0);
    println!("{}", ans);
}
