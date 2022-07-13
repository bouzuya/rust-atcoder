use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        t: usize,
    };
    let ans = (n + x - 1) / x * t;
    println!("{}", ans);
}
