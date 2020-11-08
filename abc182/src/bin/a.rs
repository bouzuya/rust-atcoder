use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = std::cmp::max(0, (2 * a + 100) - b);
    println!("{}", ans);
}
