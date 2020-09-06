use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    let ans = std::cmp::min(a * n, b);
    println!("{}", ans);
}
