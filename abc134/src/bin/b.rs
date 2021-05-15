use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
    };
    let r = 2 * d + 1;
    let ans = (n + r - 1) / r;
    println!("{}", ans);
}
