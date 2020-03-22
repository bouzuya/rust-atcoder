use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let ans = ((n + m) * (n + m - 1)) / 2 - m * n;
    println!("{}", ans);
}
