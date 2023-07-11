use proconio::input;

fn main() {
    input! {
        x: i64,
    };
    let ans = x.max(0);
    println!("{}", ans);
}
