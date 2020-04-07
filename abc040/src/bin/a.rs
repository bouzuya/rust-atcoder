use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize
    };
    let ans = std::cmp::min(n - x, x - 1);
    println!("{}", ans);
}
