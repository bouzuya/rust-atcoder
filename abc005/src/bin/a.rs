use proconio::input;

fn main() {
    input! {
        x: u64,
        y: u64,
    };
    let ans = y / x;
    println!("{}", ans);
}
