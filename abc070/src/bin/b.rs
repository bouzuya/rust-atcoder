use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };
    let ans = std::cmp::max(0, std::cmp::min(b, d) - std::cmp::max(a, c));
    println!("{}", ans);
}
