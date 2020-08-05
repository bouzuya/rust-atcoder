use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        k: i64,
        s: i64,
        t: i64,
    };
    let ans = if s + t >= k {
        s * (a - c) + t * (b - c)
    } else {
        s * a + t * b
    };
    println!("{}", ans);
}
