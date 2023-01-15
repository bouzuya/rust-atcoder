use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    };
    let ans = (n - x).min(x - 1);
    println!("{}", ans);
}
