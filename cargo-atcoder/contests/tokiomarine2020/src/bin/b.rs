use proconio::input;

fn main() {
    input! {
        a: i64,
        v: i64,
        b: i64,
        w: i64,
        t: i64,
    };
    let d = (b - a).abs();
    let dv = v - w;
    let ans = dv * t >= d;
    println!("{}", if ans { "YES" } else { "NO" });
}
