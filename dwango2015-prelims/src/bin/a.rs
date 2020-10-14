use proconio::input;

fn main() {
    input! {
        n: i64,
        x: i64,
    };
    let ans = 540 * x + 525 * (n - x);
    println!("{}", ans);
}
