use proconio::input;

fn main() {
    input! {
        n: i64,
        x: i64,
        t: i64,
    };
    let ans = ((n + x - 1) / x) * t;
    println!("{}", ans);
}
