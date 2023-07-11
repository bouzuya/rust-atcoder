use proconio::input;

fn main() {
    input! {
        x: i64,
        t: i64,
    };
    let ans = std::cmp::max(0, x - t);
    println!("{}", ans);
}
