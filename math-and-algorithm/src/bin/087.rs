use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let modp = 1_000_000_007;
    let x = (1 + n) * n / 2 % modp;
    let ans = x * x % modp;
    println!("{}", ans);
}
