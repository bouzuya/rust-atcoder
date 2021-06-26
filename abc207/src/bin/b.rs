use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };
    // a + b * x <= c * d * x
    // a <= (c * d - b) * x
    // a / (c * d - b) <= x
    if b >= c * d {
        println!("-1");
        return;
    }
    let ans = (a + (c * d - b - 1)) / (c * d - b);
    println!("{}", ans);
}
