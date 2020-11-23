use proconio::input;

fn main() {
    input! {
        x: i64,
    };
    let ans = std::cmp::max(x, 0);
    println!("{}", ans);
}
