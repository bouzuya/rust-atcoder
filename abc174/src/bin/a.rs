use proconio::input;

fn main() {
    input! {
        x: i64,
    };
    let ans = x >= 30;
    println!("{}", if ans { "Yes" } else { "No" });
}
