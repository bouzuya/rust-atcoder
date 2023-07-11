use proconio::input;

fn main() {
    input! {
        a: i64,
        d: i64,
    };

    let ans = std::cmp::max((a + 1) * d, a * (d + 1));
    println!("{}", ans);
}
