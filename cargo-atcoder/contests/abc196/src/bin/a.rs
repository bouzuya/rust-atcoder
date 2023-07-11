use proconio::input;

fn main() {
    input! {
        _a: i64,
        b: i64,
        c: i64,
        _d: i64,
    };
    let ans = b - c;
    println!("{}", ans);
}
