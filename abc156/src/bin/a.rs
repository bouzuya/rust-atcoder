use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize
    };
    let ans = r + 100 * 10usize.saturating_sub(n);
    println!("{}", ans);
}
