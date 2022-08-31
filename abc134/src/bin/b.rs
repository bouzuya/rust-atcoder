use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
    };
    let ans = (n + (2 * d + 1) - 1) / (2 * d + 1);
    println!("{}", ans);
}
