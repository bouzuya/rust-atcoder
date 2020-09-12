use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = std::cmp::min(n / 10 * 100 + n % 10 * 15, (n + 10 - 1) / 10 * 100);
    println!("{}", ans);
}
