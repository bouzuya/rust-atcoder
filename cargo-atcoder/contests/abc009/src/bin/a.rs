use proconio::input;

fn main() {
    input! {
        n: i64,
    };
    let ans = (n + (2 - 1)) / 2;
    println!("{}", ans);
}
