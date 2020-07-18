use proconio::input;

fn main() {
    input! {
        n: i64,
    };
    let x = 800 * n;
    let y = 200 * (n / 15);
    let ans = x - y;
    println!("{}", ans);
}
