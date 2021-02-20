use proconio::input;

fn main() {
    input! {
        x: i64,
    };
    let ans = 100 - x % 100;
    println!("{}", ans);
}
