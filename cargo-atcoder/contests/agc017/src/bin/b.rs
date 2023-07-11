use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };
    let ans =
        (0..n).any(|m| ((c * (n - 1 - m) - d * m)..=(d * (n - 1 - m) - c * m)).contains(&(b - a)));
    println!("{}", if ans { "YES" } else { "NO" });
}
