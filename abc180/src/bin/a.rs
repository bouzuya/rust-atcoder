use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    };
    let ans = std::cmp::max(0, n - a) + b;
    println!("{}", ans);
}
