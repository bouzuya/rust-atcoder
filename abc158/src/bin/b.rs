use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    };
    let ans = a * (n / (a + b)) + std::cmp::min(a, n % (a + b));
    println!("{}", ans);
}
