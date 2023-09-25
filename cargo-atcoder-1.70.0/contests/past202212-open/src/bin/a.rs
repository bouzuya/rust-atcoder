use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    };
    let ans = n / x * y;
    println!("{}", ans);
}
