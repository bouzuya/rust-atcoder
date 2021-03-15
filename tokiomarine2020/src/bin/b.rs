use proconio::input;

fn main() {
    input! {
        a: i64,
        v: i64,
        b: i64,
        w: i64,
        t: i64,
    };
    if v <= w {
        println!("NO");
        return;
    }
    let d = (a - b).abs();
    let v = v - w;
    let ans = v * t >= d;
    println!("{}", if ans { "YES" } else { "NO" });
}
