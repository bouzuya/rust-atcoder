use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };
    let ans = c / std::cmp::min(a, b);
    println!("{}", ans);
}
