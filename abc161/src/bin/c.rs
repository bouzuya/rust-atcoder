use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    };
    let x = n % k;
    let ans = std::cmp::min(x, k - x);
    println!("{}", ans);
}
