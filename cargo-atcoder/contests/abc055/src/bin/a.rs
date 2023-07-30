use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = 800 * n - (n / 15) * 200;
    println!("{}", ans);
}
